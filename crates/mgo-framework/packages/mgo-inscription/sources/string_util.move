module mgo_inscription::string_util {
    use std::string;
    use std::string::String;
    use std::vector;

    public fun is_empty_str(input: &String): bool {
        is_empty(string::bytes(input))
    }

    public fun is_empty(input: &vector<u8>): bool {
        vector::is_empty(input)
    }

    public fun is_tick_valid(input: &String): bool {
        let bytes = string::bytes(input);
        let len = vector::length(bytes);
        let i = 0;
        let res = len > 0;
        while (i < len) {
            let b = *vector::borrow(bytes, i);
            res = is_number(b) || is_uppercase(b);
            if (!res) {
                break
            };
            i = i + 1;
        };
        res
    }

    public fun to_uppercase(input: &mut vector<u8>) {
        let length = vector::length(input);
        let i = 0;
        while (i < length) {
            let letter = vector::borrow_mut(input, i);
            if (is_lowercase(*letter)) {
                *letter = *letter - 32;
            };
            i = i + 1;
        }
    }

    public fun to_lowercase(input: &mut vector<u8>) {
        let length = vector::length(input);
        let i = 0;
        while (i < length) {
            let letter = vector::borrow_mut(input, i);
            if (is_uppercase(*letter)) {
                *letter = *letter + 32;
            };
            i = i + 1;
        }
    }

    public fun is_number(letter: u8): bool {
        letter >= 48 && letter <= 57
    }

    public fun is_lowercase(letter: u8): bool {
        letter >= 97 && letter <= 122
    }

    public fun is_uppercase(letter: u8): bool {
        letter >= 65 && letter <= 90
    }

    public fun starts_with(input: &vector<u8>, prefix: &vector<u8>): bool {
        let input_length = vector::length(input);
        let prefix_length = vector::length(prefix);
        if (input_length < prefix_length) {
            return false
        };
        let i = 0;
        while (i < prefix_length) {
            if (vector::borrow(input, i) != vector::borrow(prefix, i)) {
                return false
            };
            i = i + 1;
        };
        true
    }

    /// Returns if the input contains the search string and the index of the first match
    public fun index_of(input: &vector<u8>, search: &vector<u8>): (bool, u64) {
        let input_length = vector::length(input);
        let search_length = vector::length(search);
        if (input_length < search_length) {
            return (false, 0)
        };
        let i = 0;
        while (i < input_length) {
            let j = 0;
            while (j < search_length) {
                let idx = i + j;
                if (idx >= input_length) {
                    break
                };
                if (vector::borrow(input, idx) != vector::borrow(search, j)) {
                    break
                };
                j = j + 1;
            };
            if (j == search_length) {
                return (true, i)
            };
            i = i + 1;
        };
        (false, 0)
    }

    /// Returns if the input contains the search string and the index of the last match
    public fun last_index_of(input: &vector<u8>, search: &vector<u8>): (bool, u64) {
        let input_length = vector::length(input);
        let search_length = vector::length(search);
        if (input_length < search_length) {
            return (false, 0)
        };
        let i = input_length - search_length;
        while (i >= 0) {
            let j = 0;
            while (j < search_length) {
                let idx = i + j;
                if (idx >= input_length) {
                    break
                };
                if (vector::borrow(input, idx) != vector::borrow(search, j)) {
                    break
                };
                j = j + 1;
            };
            if (j == search_length) {
                return (true, i)
            };
            if (i == 0) {
                break
            };
            i = i - 1;
        };
        (false, 0)
    }

    public fun substring(input: &vector<u8>, start: u64, end: u64): vector<u8> {
        let length = vector::length(input);
        if (start >= length) {
            return vector::empty()
        };
        let end = if (end > length) {
            length
        } else {
            end
        };
        let result = vector::empty();
        let i = start;
        while (i < end) {
            vector::push_back(&mut result, *vector::borrow(input, i));
            i = i + 1;
        };
        result
    }

    /// Returns if the input contains any of the chars
    public fun contains_any(input: &vector<u8>, chars: &vector<u8>): bool {
        let length = vector::length(input);
        let chars_length = vector::length(chars);
        let i = 0;
        while (i < length) {
            let j = 0;
            while (j < chars_length) {
                if (vector::borrow(input, i) == vector::borrow(chars, j)) {
                    return true
                };
                j = j + 1;
            };
            i = i + 1;
        };
        false
    }
}