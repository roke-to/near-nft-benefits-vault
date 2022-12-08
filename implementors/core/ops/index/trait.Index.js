(function() {var implementors = {
"near_sdk":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.u32.html\">u32</a>&gt; for <a class=\"struct\" href=\"near_sdk/store/vec/struct.Vector.html\" title=\"struct near_sdk::store::vec::Vector\">Vector</a>&lt;T&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BorshSerialize + BorshDeserialize,</span>"],["impl&lt;K, V, H, Q:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.reference.html\">&amp;</a>Q&gt; for <a class=\"struct\" href=\"near_sdk/store/lookup_map/struct.LookupMap.html\" title=\"struct near_sdk::store::lookup_map::LookupMap\">LookupMap</a>&lt;K, V, H&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: BorshSerialize + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;Q&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: BorshSerialize + BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;H: <a class=\"trait\" href=\"near_sdk/store/key/trait.ToKey.html\" title=\"trait near_sdk::store::key::ToKey\">ToKey</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Q: BorshSerialize + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/alloc/borrow/trait.ToOwned.html\" title=\"trait alloc::borrow::ToOwned\">ToOwned</a>&lt;Owned = K&gt;,</span>"],["impl&lt;K, V, H, Q:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.65.0/std/primitive.reference.html\">&amp;</a>Q&gt; for <a class=\"struct\" href=\"near_sdk/store/unordered_map/struct.UnorderedMap.html\" title=\"struct near_sdk::store::unordered_map::UnorderedMap\">UnorderedMap</a>&lt;K, V, H&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: BorshSerialize + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/cmp/trait.Ord.html\" title=\"trait core::cmp::Ord\">Ord</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;Q&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: BorshSerialize + BorshDeserialize,<br>&nbsp;&nbsp;&nbsp;&nbsp;H: <a class=\"trait\" href=\"near_sdk/store/key/trait.ToKey.html\" title=\"trait near_sdk::store::key::ToKey\">ToKey</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Q: BorshSerialize + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.65.0/alloc/borrow/trait.ToOwned.html\" title=\"trait alloc::borrow::ToOwned\">ToOwned</a>&lt;Owned = K&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()