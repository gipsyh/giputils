#pragma once
#include <bits/stdc++.h>
extern "C" {
void rust_vec_drop(struct RawRustVec);
}

struct RawRustVec {
	size_t cap;
	void *ptr;
	size_t len;
	size_t sizet;
};

class RustVec {
    public:
	~RustVec()
	{
		rust_vec_drop(raw);
	}

	RawRustVec raw;
};
