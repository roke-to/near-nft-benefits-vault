(function() {var implementors = {};
implementors["near_sdk"] = [{"text":"impl&lt;K, V&gt; BorshSerialize for <a class=\"struct\" href=\"near_sdk/collections/struct.LegacyTreeMap.html\" title=\"struct near_sdk::collections::LegacyTreeMap\">LegacyTreeMap</a>&lt;K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u64.html\">u64</a>: BorshSerialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"near_sdk/collections/unordered_map/struct.UnorderedMap.html\" title=\"struct near_sdk::collections::unordered_map::UnorderedMap\">UnorderedMap</a>&lt;K, V&gt;: BorshSerialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"near_sdk/collections/vector/struct.Vector.html\" title=\"struct near_sdk::collections::vector::Vector\">Vector</a>&lt;Node&lt;K&gt;&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::collections::legacy_tree_map::LegacyTreeMap"]},{"text":"impl&lt;K, V&gt; BorshSerialize for <a class=\"struct\" href=\"near_sdk/collections/struct.LookupMap.html\" title=\"struct near_sdk::collections::LookupMap\">LookupMap</a>&lt;K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::collections::lookup_map::LookupMap"]},{"text":"impl&lt;T&gt; BorshSerialize for <a class=\"struct\" href=\"near_sdk/collections/struct.LookupSet.html\" title=\"struct near_sdk::collections::LookupSet\">LookupSet</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::collections::lookup_set::LookupSet"]},{"text":"impl&lt;T&gt; BorshSerialize for <a class=\"struct\" href=\"near_sdk/collections/vector/struct.Vector.html\" title=\"struct near_sdk::collections::vector::Vector\">Vector</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u64.html\">u64</a>: BorshSerialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::collections::vector::Vector"]},{"text":"impl&lt;K, V&gt; BorshSerialize for <a class=\"struct\" href=\"near_sdk/collections/unordered_map/struct.UnorderedMap.html\" title=\"struct near_sdk::collections::unordered_map::UnorderedMap\">UnorderedMap</a>&lt;K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;: BorshSerialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"near_sdk/collections/vector/struct.Vector.html\" title=\"struct near_sdk::collections::vector::Vector\">Vector</a>&lt;K&gt;: BorshSerialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"near_sdk/collections/vector/struct.Vector.html\" title=\"struct near_sdk::collections::vector::Vector\">Vector</a>&lt;V&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::collections::unordered_map::UnorderedMap"]},{"text":"impl&lt;T&gt; BorshSerialize for <a class=\"struct\" href=\"near_sdk/collections/struct.UnorderedSet.html\" title=\"struct near_sdk::collections::UnorderedSet\">UnorderedSet</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;: BorshSerialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"near_sdk/collections/vector/struct.Vector.html\" title=\"struct near_sdk::collections::vector::Vector\">Vector</a>&lt;T&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::collections::unordered_set::UnorderedSet"]},{"text":"impl&lt;T&gt; BorshSerialize for <a class=\"struct\" href=\"near_sdk/collections/struct.LazyOption.html\" title=\"struct near_sdk::collections::LazyOption\">LazyOption</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::collections::lazy_option::LazyOption"]},{"text":"impl&lt;K, V&gt; BorshSerialize for <a class=\"struct\" href=\"near_sdk/collections/struct.TreeMap.html\" title=\"struct near_sdk::collections::TreeMap\">TreeMap</a>&lt;K, V&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u64.html\">u64</a>: BorshSerialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"near_sdk/collections/struct.LookupMap.html\" title=\"struct near_sdk::collections::LookupMap\">LookupMap</a>&lt;K, V&gt;: BorshSerialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"near_sdk/collections/vector/struct.Vector.html\" title=\"struct near_sdk::collections::vector::Vector\">Vector</a>&lt;Node&lt;K&gt;&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::collections::tree_map::TreeMap"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/struct.Promise.html\" title=\"struct near_sdk::Promise\">Promise</a>","synthetic":false,"types":["near_sdk::promise::Promise"]},{"text":"impl&lt;T:&nbsp;BorshSerialize&gt; BorshSerialize for <a class=\"enum\" href=\"near_sdk/enum.PromiseOrValue.html\" title=\"enum near_sdk::PromiseOrValue\">PromiseOrValue</a>&lt;T&gt;","synthetic":false,"types":["near_sdk::promise::PromiseOrValue"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/json_types/struct.Base58CryptoHash.html\" title=\"struct near_sdk::json_types::Base58CryptoHash\">Base58CryptoHash</a>","synthetic":false,"types":["near_sdk::json_types::hash::Base58CryptoHash"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/json_types/struct.U128.html\" title=\"struct near_sdk::json_types::U128\">U128</a>","synthetic":false,"types":["near_sdk::json_types::integers::U128"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/json_types/struct.U64.html\" title=\"struct near_sdk::json_types::U64\">U64</a>","synthetic":false,"types":["near_sdk::json_types::integers::U64"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/json_types/struct.I128.html\" title=\"struct near_sdk::json_types::I128\">I128</a>","synthetic":false,"types":["near_sdk::json_types::integers::I128"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/json_types/struct.I64.html\" title=\"struct near_sdk::json_types::I64\">I64</a>","synthetic":false,"types":["near_sdk::json_types::integers::I64"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/json_types/struct.Base64VecU8.html\" title=\"struct near_sdk::json_types::Base64VecU8\">Base64VecU8</a>","synthetic":false,"types":["near_sdk::json_types::vector::Base64VecU8"]},{"text":"impl BorshSerialize for <a class=\"enum\" href=\"near_sdk/json_types/enum.CurveType.html\" title=\"enum near_sdk::json_types::CurveType\">CurveType</a>","synthetic":false,"types":["near_sdk::types::public_key::CurveType"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/struct.PublicKey.html\" title=\"struct near_sdk::PublicKey\">PublicKey</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["near_sdk::types::public_key::PublicKey"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/struct.AccountId.html\" title=\"struct near_sdk::AccountId\">AccountId</a>","synthetic":false,"types":["near_sdk::types::account_id::AccountId"]},{"text":"impl BorshSerialize for <a class=\"struct\" href=\"near_sdk/struct.Gas.html\" title=\"struct near_sdk::Gas\">Gas</a>","synthetic":false,"types":["near_sdk::types::gas::Gas"]}];
implementors["nft_benefits_vault"] = [{"text":"impl BorshSerialize for <a class=\"struct\" href=\"nft_benefits_vault/struct.Contract.html\" title=\"struct nft_benefits_vault::Contract\">Contract</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"near_sdk/collections/unordered_map/struct.UnorderedMap.html\" title=\"struct near_sdk::collections::unordered_map::UnorderedMap\">UnorderedMap</a>&lt;NftId, Vault&gt;: BorshSerialize,&nbsp;</span>","synthetic":false,"types":["nft_benefits_vault::Contract"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()