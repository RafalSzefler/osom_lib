The Bytell Hash Table
=====================

The bytell (byte linked list) hash table is an algorithm by Malte Skarupke,
which we implement here in Rust.

The overview of the algorithm can be found here: [A new fast hash table in response to Google’s new fast hash table](https://probablydance.com/2018/05/28/a-new-fast-hash-table-in-response-to-googles-new-fast-hash-table/)

And Matle Skarupke's C++ implementation can be found in he's
[GitHub repository](https://github.com/skarupke/flat_hash_map).

Properties
----------

The bytell hash table is optimized for memory usage, it uses 1 byte overhead
per entry. Insetion, query and removal are quite fast as well.

Basic layout
------------

Bytell hash table internally is built from a contiguous sequence of blocks.
The number of blocks is always a power of two, at any time.

Each block is composed of `N` entries, where `N` is also a power of two, and it
is fixed, depending on key and value types only. Typically `N` is a low number like 8 or 16.

A block entry is a triple `(control_byte, key, value)`, where `control_byte` is
a single byte (explained later), `key` is a comparable, hashable key and `value`
is any value.

The exact layout of block **is not** a contiguous sequence of `(control_byte, key, value)`
triples. On the contrary, we first store a sequence of `N` control bytes, and then
a sequence of `N` key-value pairs. In particular `N` has to be at least the alignment
of `(key, value)` pair (in reality it is at least 8 as well).

The `capacity` of bytell hash table is calculated as `number_of_blocks * N`. And so
it is a power of two as well.

Additionally a hash function `hash(x)` is attached to the bytell hash table. And also
a `hash_to_index(x)` function, which turns the hashed value into an index usable by
the bytell hash table internally. The `hash_to_index(x)` value is guaranteed to be
in `0..capacity` range.

control_byte
------------

The `control_byte` is a byte (8-bit unsigned number) which is divided into two pieces:
1. The first bit, if set, denotes that the entry is "direct", meaning some hash maps
   exactly to this entry. If it is zero, then it is a "storage" entry, meaning it is used
   for storing a linked list of (key, value) pairs that share the hash. We will call
   this field `control_byte.type`.
2. The remaining 7 bits are used as jump distance indexes. Given such an entry with non-zero
   jump distance index we can calculate the next item in the linked list by calculating an
   appropriate offset based on the index. So this field is a 7-bit integer denoted by
   `control_byte.distance_index`.

The `distance_index` is not an offset to the next entry. There's an indirection: the
`distance_index` retrieves the offset from a global static `JUMP_DISTANCES` array, where
each distance is carefuly chosen for the highest quality.

Nevertheless, the `next_entry` that follows `prev_entry` through `distance_index` will
be denoted by `next_entry := prev_entry + prev_entry.control_byte.distance_index`. We
keep in mind that in reality there's this `JUMP_DISTANCES` array indirection.

Query
-----

For a given `query_key` the retrieval of the value works through the following steps:

* Let `hash_value := hash(query_key)`
* Let `index := hash_to_index(hash_value)`
* Let `block := blocks[index % number_of_blocks]`
* Let `entry := block.entries[index % N]`
* If `entry.control_byte.type != DIRECT_HIT` then return `NONE`
* If `entry.control_byte.type == DIRECT_HIT` then:
  * loop:
    * If `entry.key == query_key` then return `(entry.key, entry.value)`
    * If `entry.control_byte.distance_index == 0` then return `NONE`
    * `entry := entry + entry.control_byte.distance_index`

Delete
------

For a given `delete_key` the deletion of a key works like this:

* Let `hash_value := hash(delete_key)`
* Let `index := hash_to_index(hash_value)`
* Let `block := blocks[index % number_of_blocks]`
* Let `entry := block.entries[index % N]`
* If `entry.control_byte.type != DIRECT_HIT` then return `NONE`
* loop:
  * If `entry.key == delete_key` then break with `found_entry := entry`
  * If `entry.control_byte.distance_index == 0` then return `NONE`
  * `entry := entry + entry.control_byte.distance_index`
* We found matching entry. But in order to delete it, we first swap it with the last
  in the linked list.
* Let `tail := last_linked_entry(found_entry)`
* If `tail != found_entry` then `swap(tail, found_entry)`. Now `found_entry`
  is at the end of the list, it has no successor.
* Let `parent_entry := parent(found_entry)`
* `parent_entry.control_byte.distance_index := 0`
* `found_entry.control_byte := EMPTY`
* Return `found_entry`

Insert
------

For a given `(insert_key, insert_value)` pair the insertion works like this:

* Let `hash_value := hash(insert_key)`
* Let `index := hash_to_index(hash_value)`
* Let `block := blocks[index % number_of_blocks]`
* Let `entry := block.entries[index % N]`
* If `entry.control_byte.type == DIRECT_HIT`:
  * loop "direct":
    * If `entry.key == insert_key` then `entry.value := insert_value` and return `old_value`.
      We did an in-place update.
    * If `entry.control_byte.distance_index != 0` then:
      * `entry := entry + entry.control_byte.distance_index`
      * continue loop "direct"
    * We reached the end of the linked list. We need to insert a new element.
    * If `should_grow()` then grow the table
    * loop "inner":
      * Let `(free_entry, free_entry_distance) := search_for_free_entry(entry)`
      * If found then break "inner", otherwise grow the table. Panic if that is not possible.
    * `entry.control_byte.distance_index := free_entry_distance`
    * `free_entry.control_byte.type := STORAGE`
    * `free_entry.control_byte.distance_index := 0`
    * `free_entry.key := insert_key`
    * `free_entry.value := value`
    * RETURN
* If `entry.control_byte.type != DIRECT_HIT`:
  * We have direct hit on an entry that is actually a storage entry. What
    we need to do is to move around values to make place for this hit.
  * Let `parent_entry := parent(entry)`
  * Let `tmp_current := entry` and `entry := reserved`. We reserve the entry,
    because we don't know the operation of moving around stuff to potentially hit it again.
  * Let `original_entry := entry`
  * loop "outer":
    * loop "inner":
      * Let `(free_entry, free_entry_distance) := search_for_free_entry(entry)`
      * If found then break "inner", otherwise grow the table. Panic if that is not possible
    * copy `tmp_current` to `free_entry`, set distance on `parent_entry`
    * If `tmp_current.control_byte.distance_index == 0` then break "outer"
    * `parent_entry := tmp_current`, and `tmp_current := tmp_current + tmp_current.control_byte.distance_index`
  * Now we have place for a direct hit
  * `original_entry.control_byte.type := DIRECT_HIT`
  * `original_entry.control_byte.distance_index := 0`
  * `original_entry.key := insert_key`
  * `original_entry.value := insert_value`
  * RETURN
