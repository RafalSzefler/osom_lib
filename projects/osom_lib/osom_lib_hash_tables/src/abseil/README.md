The Abseil Hash Table
=====================

The Abseil hash table is our port of the [Google's Abseil flat_hash_table](https://abseil.io/docs/cpp/guides/container).

Basic layout
------------

The table internally stores an additional `control_byte` of size 1 for each `(key, value)` pair.
The layout is as follows: a contiguous block of all `control_byte` bytes, followed by padding, and
finally followed by a contiguous block of all `(key, value)` pairs, a.k.a. buckets.

control_byte
------------

The `control_byte` is a byte (8-bit unsigned number) which is divided into two pieces:
1. The first bit, if set to `0`, denotes that the entry exists, meaning that the corresponding
   bucket holds an actual `(key, value)` pair. Then remaining 7 bits are filled with
   `hash(key) & 0b1111111` value.
2. Otherwise `control_byte` can have one of two special values: `0b10000000 = 0x80` which represents
   an empty (i.e. ready for insertion) bucket and `0b11111111 == 0xFF` which represents a deleted bucket,
   a.k.a. tombstone.

The above values are deliberately chosen to make SIMD operations easy.

Note
----

All the SIMD operations are done over the length of 16. Whenever you see a magical "16"
appearing then it is likely it corresponds to the length of a SIMD group.

Quadratic probing
-----------------

All the algorithms will utilize quadratic probing iterator `quad(length)`, which
in practice is instantiated as a struct with two fields: `idx` and `step` both initialized
to zero. The struct has one method `next()` which yields current value of `step` and then
increments `idx += 1` and `step += idx`. The iterator yields value `length` number of times.

Initial setup
-------------

All the algorithms start with the same initial setup steps:

* Let `group_count := table_length / 16`
* Let `hash_value := hash(query_key)`
* Let `h1 := hash_value >> 7`
* Let `group_index := h1 % group_count`
* Let `h2 := hash_value & 127`
* Let `simd_query := (h1, ..., h2) of length 16`
* Let `prober := quad(group_count)`

Query
-----

For a given `query_key` the retrieval of the value works through the following steps:

* loop until `prober` yields values:
  * Let `group_idx := (h1 + prober.next()) % group_count`
  * Let `control_byte_ptr := control_bytes[group_idx]`.
  * Let `matches := simd_match(control_byte_ptr, simd_query)`
  * If no `matches` then return `NONE`
  * For each `match` in `matches` get the concrete bucket idx and compare the corresponding
    `key` with `query_key`. If they match return `matched_entry`. Otherwise continue.
* (should not be reachable)

Delete
------

For a given `delete_key` the deletion of a key works like this:

* loop until `prober` yields values:
  * Let `group_idx := (h1 + prober.next()) % group_count`
  * Let `control_byte_ptr := control_bytes[group_idx]`.
  * Let `matches := simd_match(control_byte_ptr, simd_query)`
  * If no `matches` then return `NONE`
  * For each `match` in `matches` get the concrete bucket idx and compare the corresponding
    `key` with `delete_key`. Continue if no match.
  * If we actually have a match, then set `control_bytes[group_idx + match_idx] := 0xFF`
    and return `matched_entry`.
* (should not be reachable)

Insert
------

For a given `(insert_key, insert_value)` pair the insertion works like this:

* Let `first_deleted := NONE` (we will remember the first tombstone seen while probing).
* loop until `prober` yields values:
  * Let `group_idx := (h1 + prober.next()) % group_count`
  * Let `control_byte_ptr := control_bytes[group_idx]`.
  * Let `matches := simd_match(control_byte_ptr, simd_query)` (candidate buckets with matching `h2`).
  * For each `match` in `matches`:
    * Let `bucket_idx := group_idx * 16 + match`
    * Compare `bucket[bucket_idx].key` with `insert_key`.
    * If keys match, replace the value in-place and return `old_value`.
* No existing key was found. We now search for an insertion slot:
  * loop until `prober` yields values:
    * Let `group_idx := (h1 + prober.next()) % group_count`
    * Let `control_byte_ptr := control_bytes[group_idx]`.
    * Let `deleted_matches := simd_match(control_byte_ptr, 0xFF)`
    * If `first_deleted == NONE` and `deleted_matches` is non-empty, store the first such slot in `first_deleted`.
    * Let `empty_matches := simd_match(control_byte_ptr, 0x80)`
    * If `empty_matches` is non-empty:
      * Let `target_idx := first_deleted` if it exists, otherwise the first `empty_match`.
      * Write `bucket[target_idx] := (insert_key, insert_value)`.
      * Set `control_bytes[target_idx] := h2`.
      * Return `NONE` (new insertion).
* If probing exhausted without finding an empty bucket, grow/rebuild the table and retry insertion.
