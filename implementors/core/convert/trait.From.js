(function() {var implementors = {};
implementors["near_sdk"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;AccountId&gt; for <a class=\"struct\" href=\"near_sdk/struct.AccountId.html\" title=\"struct near_sdk::AccountId\">AccountId</a>","synthetic":false,"types":["near_sdk::types::account_id::AccountId"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/struct.Promise.html\" title=\"struct near_sdk::Promise\">Promise</a>&gt; for <a class=\"enum\" href=\"near_sdk/enum.PromiseOrValue.html\" title=\"enum near_sdk::PromiseOrValue\">PromiseOrValue</a>&lt;T&gt;","synthetic":false,"types":["near_sdk::promise::PromiseOrValue"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/json_types/struct.Base58CryptoHash.html\" title=\"struct near_sdk::json_types::Base58CryptoHash\">Base58CryptoHash</a>&gt; for <a class=\"type\" href=\"near_sdk/type.CryptoHash.html\" title=\"type near_sdk::CryptoHash\">CryptoHash</a>","synthetic":false,"types":["near_sdk::types::CryptoHash"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.array.html\">; 32]</a>&gt; for <a class=\"struct\" href=\"near_sdk/json_types/struct.Base58CryptoHash.html\" title=\"struct near_sdk::json_types::Base58CryptoHash\">Base58CryptoHash</a>","synthetic":false,"types":["near_sdk::json_types::hash::Base58CryptoHash"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"near_sdk/json_types/struct.Base58CryptoHash.html\" title=\"struct near_sdk::json_types::Base58CryptoHash\">Base58CryptoHash</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>","synthetic":false,"types":["alloc::string::String"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u128.html\">u128</a>&gt; for <a class=\"struct\" href=\"near_sdk/json_types/struct.U128.html\" title=\"struct near_sdk::json_types::U128\">U128</a>","synthetic":false,"types":["near_sdk::json_types::integers::U128"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/json_types/struct.U128.html\" title=\"struct near_sdk::json_types::U128\">U128</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u128.html\">u128</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"near_sdk/json_types/struct.U64.html\" title=\"struct near_sdk::json_types::U64\">U64</a>","synthetic":false,"types":["near_sdk::json_types::integers::U64"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/json_types/struct.U64.html\" title=\"struct near_sdk::json_types::U64\">U64</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u64.html\">u64</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.i128.html\">i128</a>&gt; for <a class=\"struct\" href=\"near_sdk/json_types/struct.I128.html\" title=\"struct near_sdk::json_types::I128\">I128</a>","synthetic":false,"types":["near_sdk::json_types::integers::I128"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/json_types/struct.I128.html\" title=\"struct near_sdk::json_types::I128\">I128</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.i128.html\">i128</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.i64.html\">i64</a>&gt; for <a class=\"struct\" href=\"near_sdk/json_types/struct.I64.html\" title=\"struct near_sdk::json_types::I64\">I64</a>","synthetic":false,"types":["near_sdk::json_types::integers::I64"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/json_types/struct.I64.html\" title=\"struct near_sdk::json_types::I64\">I64</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.i64.html\">i64</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"struct\" href=\"near_sdk/json_types/struct.Base64VecU8.html\" title=\"struct near_sdk::json_types::Base64VecU8\">Base64VecU8</a>","synthetic":false,"types":["near_sdk::json_types::vector::Base64VecU8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/json_types/struct.Base64VecU8.html\" title=\"struct near_sdk::json_types::Base64VecU8\">Base64VecU8</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;","synthetic":false,"types":["alloc::vec::Vec"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"near_sdk/enum.PromiseResult.html\" title=\"enum near_sdk::PromiseResult\">PromiseResult</a>&gt; for <a class=\"enum\" href=\"near_sdk/enum.VmPromiseResult.html\" title=\"enum near_sdk::VmPromiseResult\">VmPromiseResult</a>","synthetic":false,"types":["near_vm_logic::types::PromiseResult"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/struct.PublicKey.html\" title=\"struct near_sdk::PublicKey\">PublicKey</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u8.html\">u8</a>&gt;","synthetic":false,"types":["alloc::vec::Vec"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"near_sdk/struct.PublicKey.html\" title=\"struct near_sdk::PublicKey\">PublicKey</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>","synthetic":false,"types":["alloc::string::String"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/struct.AccountId.html\" title=\"struct near_sdk::AccountId\">AccountId</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.64.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>","synthetic":false,"types":["alloc::string::String"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"near_sdk/struct.Gas.html\" title=\"struct near_sdk::Gas\">Gas</a>","synthetic":false,"types":["near_sdk::types::gas::Gas"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"near_sdk/struct.Gas.html\" title=\"struct near_sdk::Gas\">Gas</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.64.0/std/primitive.u64.html\">u64</a>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()