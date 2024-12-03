///Module containing a contract's types and functions.
/**

```solidity
library IServerAllocator {
    struct NonceConsumption { address signer; uint256[] nonces; bytes32[] attests; }
    struct RegisterAttest { address signer; bytes32 attestHash; uint256 expiration; uint256 nonce; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IServerAllocator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct NonceConsumption { address signer; uint256[] nonces; bytes32[] attests; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonceConsumption {
        pub signer: alloy::sol_types::private::Address,
        pub nonces: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub attests: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NonceConsumption> for UnderlyingRustTuple<'_> {
            fn from(value: NonceConsumption) -> Self {
                (value.signer, value.nonces, value.attests)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonceConsumption {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    nonces: tuple.1,
                    attests: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for NonceConsumption {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for NonceConsumption {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonces),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.attests),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for NonceConsumption {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for NonceConsumption {
            const NAME: &'static str = "NonceConsumption";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "NonceConsumption(address signer,uint256[] nonces,bytes32[] attests)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonces)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.attests)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for NonceConsumption {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signer,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonces,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.attests,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signer,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonces,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.attests,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct RegisterAttest { address signer; bytes32 attestHash; uint256 expiration; uint256 nonce; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RegisterAttest {
        pub signer: alloy::sol_types::private::Address,
        pub attestHash: alloy::sol_types::private::FixedBytes<32>,
        pub expiration: alloy::sol_types::private::primitives::aliases::U256,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RegisterAttest> for UnderlyingRustTuple<'_> {
            fn from(value: RegisterAttest) -> Self {
                (value.signer, value.attestHash, value.expiration, value.nonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RegisterAttest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signer: tuple.0,
                    attestHash: tuple.1,
                    expiration: tuple.2,
                    nonce: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RegisterAttest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RegisterAttest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.attestHash),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiration),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RegisterAttest {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RegisterAttest {
            const NAME: &'static str = "RegisterAttest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RegisterAttest(address signer,bytes32 attestHash,uint256 expiration,uint256 nonce)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.attestHash)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiration)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RegisterAttest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signer,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.attestHash,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiration,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signer,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.attestHash,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiration,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IServerAllocator`](self) contract instance.

See the [wrapper's documentation](`IServerAllocatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IServerAllocatorInstance<T, P, N> {
        IServerAllocatorInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IServerAllocator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IServerAllocator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IServerAllocatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IServerAllocatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IServerAllocatorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IServerAllocatorInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IServerAllocator`](self) contract instance.

See the [wrapper's documentation](`IServerAllocatorInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IServerAllocatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IServerAllocatorInstance<T, P, N> {
            IServerAllocatorInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IServerAllocatorInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IServerAllocatorInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IServerAllocator {
    struct NonceConsumption {
        address signer;
        uint256[] nonces;
        bytes32[] attests;
    }
    struct RegisterAttest {
        address signer;
        bytes32 attestHash;
        uint256 expiration;
        uint256 nonce;
    }
}

interface ServerAllocator {
    error AlreadyUsedSig(bytes32 attest_, uint256 nonce);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error Expired(uint256 expiration_, uint256 currentTimestamp_);
    error ExpiredAttests(bytes32 attest_);
    error InvalidCaller(address caller_, address expected_);
    error InvalidInput();
    error InvalidShortString();
    error InvalidSignature(bytes signature_, address signer_);
    error InvalidSigner(address signer_);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error StringTooLong(string str);
    error UnregisteredAttest(bytes32 attest_);

    event AttestRegistered(bytes32 attest_, uint256 expiration_);
    event Attested(address from_, uint256 id_, uint256 amount_);
    event EIP712DomainChanged();
    event NoncesConsumed(uint256[] nonces_);
    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event SignerAdded(address signer_);
    event SignerRemoved(address signer_);

    constructor(address owner_, address compactContract_);

    function acceptOwnership() external;
    function addSigner(address signer_) external;
    function attest(address, address from_, address, uint256 id_, uint256 amount_) external returns (bytes4);
    function checkAttestExpirations(bytes32 attest_) external view returns (uint256[] memory);
    function checkAttestExpirations(address sponsor_, uint256 id_, uint256 amount_) external view returns (uint256[] memory);
    function checkIfSigner(address signer_) external view returns (bool);
    function consume(uint256[] memory nonces_, bytes32[] memory attests_) external;
    function consumeViaSignature(IServerAllocator.NonceConsumption memory data_, bytes memory signature_) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getAllSigners() external view returns (address[] memory);
    function getCompactContract() external view returns (address);
    function isValidSignature(bytes32 hash_, bytes memory signature_) external view returns (bytes4 magicValue);
    function owner() external view returns (address);
    function pendingOwner() external view returns (address);
    function registerAttest(bytes32 attest_, uint256 expiration_) external;
    function registerAttestViaSignature(IServerAllocator.RegisterAttest memory attest_, bytes memory signature_) external;
    function removeSigner(address signer_) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "owner_",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "compactContract_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "acceptOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSigner",
    "inputs": [
      {
        "name": "signer_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "attest",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "from_",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id_",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkAttestExpirations",
    "inputs": [
      {
        "name": "attest_",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkAttestExpirations",
    "inputs": [
      {
        "name": "sponsor_",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id_",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkIfSigner",
    "inputs": [
      {
        "name": "signer_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "consume",
    "inputs": [
      {
        "name": "nonces_",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "attests_",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "consumeViaSignature",
    "inputs": [
      {
        "name": "data_",
        "type": "tuple",
        "internalType": "struct IServerAllocator.NonceConsumption",
        "components": [
          {
            "name": "signer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonces",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "attests",
            "type": "bytes32[]",
            "internalType": "bytes32[]"
          }
        ]
      },
      {
        "name": "signature_",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllSigners",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCompactContract",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "hash_",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "signature_",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "magicValue",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pendingOwner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerAttest",
    "inputs": [
      {
        "name": "attest_",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "expiration_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerAttestViaSignature",
    "inputs": [
      {
        "name": "attest_",
        "type": "tuple",
        "internalType": "struct IServerAllocator.RegisterAttest",
        "components": [
          {
            "name": "signer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "attestHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiration",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "signature_",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeSigner",
    "inputs": [
      {
        "name": "signer_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "AttestRegistered",
    "inputs": [
      {
        "name": "attest_",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "expiration_",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Attested",
    "inputs": [
      {
        "name": "from_",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "id_",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amount_",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NoncesConsumed",
    "inputs": [
      {
        "name": "nonces_",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferStarted",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SignerAdded",
    "inputs": [
      {
        "name": "signer_",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SignerRemoved",
    "inputs": [
      {
        "name": "signer_",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyUsedSig",
    "inputs": [
      {
        "name": "attest_",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "Expired",
    "inputs": [
      {
        "name": "expiration_",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "currentTimestamp_",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ExpiredAttests",
    "inputs": [
      {
        "name": "attest_",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidCaller",
    "inputs": [
      {
        "name": "caller_",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "expected_",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidInput",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": [
      {
        "name": "signature_",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signer_",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidSigner",
    "inputs": [
      {
        "name": "signer_",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "StringTooLong",
    "inputs": [
      {
        "name": "str",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnregisteredAttest",
    "inputs": [
      {
        "name": "attest_",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ServerAllocator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610180806040523461025057604081611dab8038038091610020828561027a565b8339810103126102505761003f6020610038836102b1565b92016102b1565b906040519161004f60408461027a565b6009835260208301916820b63637b1b0ba37b960b91b83526040519061007660408361027a565b60018252603160f81b602083019081526001600160a01b0390911693841561026757600180546001600160a01b03199081169091555f8054918216871781559660209690916001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08980a36100f1816102c5565b610120526100fe84610460565b61014052519020918260e05251902080610100524660a05260405190848201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8452604083015260608201524660808201523060a082015260a0815261016660c08261027a565b5190206080523060c0528061016052606460405180948193632a9c4d0d60e01b83523060048401526040602484015281604484015260018060a01b03165af1801561025c57610219575b6040516118129081610599823960805181611722015260a051816117df015260c051816116ec015260e05181611771015261010051816117970152610120518161060e0152610140518161063a01526101605181818161028b01528181610d54015261119e0152f35b6020813d602011610254575b816102326020938361027a565b8101031261025057516001600160601b03811603610250575f6101b0565b5f80fd5b3d9150610225565b6040513d5f823e3d90fd5b631e4fbdf760e01b5f525f60045260245ffd5b601f909101601f19168101906001600160401b0382119082101761029d57604052565b634e487b7160e01b5f52604160045260245ffd5b51906001600160a01b038216820361025057565b908151602081105f1461033f575090601f8151116102ff5760208151910151602082106102f0571790565b5f198260200360031b1b161790565b604460209160405192839163305a27a960e01b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fd5b6001600160401b03811161029d57600254600181811c91168015610456575b602082101461044257601f811161040f575b50602092601f82116001146103ae57928192935f926103a3575b50508160011b915f199060031b1c19161760025560ff90565b015190505f8061038a565b601f1982169360025f52805f20915f5b8681106103f757508360019596106103df575b505050811b0160025560ff90565b01515f1960f88460031b161c191690555f80806103d1565b919260206001819286850151815501940192016103be565b60025f52601f60205f20910160051c810190601f830160051c015b8181106104375750610370565b5f815560010161042a565b634e487b7160e01b5f52602260045260245ffd5b90607f169061035e565b908151602081105f1461048b575090601f8151116102ff5760208151910151602082106102f0571790565b6001600160401b03811161029d57600354600181811c9116801561058e575b602082101461044257601f811161055b575b50602092601f82116001146104fa57928192935f926104ef575b50508160011b915f199060031b1c19161760035560ff90565b015190505f806104d6565b601f1982169360035f52805f20915f5b868110610543575083600195961061052b575b505050811b0160035560ff90565b01515f1960f88460031b161c191690555f808061051d565b9192602060018192868501518155019401920161050a565b60035f52601f60205f20910160051c810190601f830160051c015b81811061058357506104bc565b5f8155600101610576565b90607f16906104aa56fe60806040526004361015610011575f80fd5b5f3560e01c80630e316ab714610a745780631626ba7e146109fa5780631a808f911461099e578063254204c51461097f5780632bca447f146107de5780632df97546146107ab578063715018a61461074857806379ba5097146106c357806384b0196e146105f65780638da5cb5b146105cf578063c9d0fa8614610535578063cfde437414610382578063d42f2f35146102ba578063d6996b6e14610276578063e30c39781461024e578063eb12d61e14610223578063f2fde38b146101b1578063f780c0d51461016d5763fc79101e146100ea575f80fd5b3461016957606036600319011261016957610165610151610109610a9d565b604080516001600160a01b03909216602083019081526024359183019190915260443560608301529061014981608081015b03601f198101835282610d0d565b519020611080565b604051918291602083526020830190610af7565b0390f35b5f80fd5b346101695760203660031901126101695760206101a761018b610a9d565b6001600160a01b03165f90815260046020526040902054151590565b6040519015158152f35b34610169576020366003190112610169576101ca610a9d565b6101d2611027565b60018060a01b0316806bffffffffffffffffffffffff60a01b600154161760015560018060a01b035f54167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e227005f80a3005b346101695760203660031901126101695761024c61023f610a9d565b610247611027565b610f9d565b005b34610169575f366003190112610169576001546040516001600160a01b039091168152602090f35b34610169575f366003190112610169576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610169575f366003190112610169576040518060206005549283815201809260055f527f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db0905f5b8181106103635750505081610318910382610d0d565b604051918291602083019060208452518091526040830191905f5b818110610341575050500390f35b82516001600160a01b0316845285945060209384019390920191600101610333565b82546001600160a01b0316845260209093019260019283019201610302565b3461016957366003190160a08112610169576080136101695760843567ffffffffffffffff8111610169576103bb903690600401610ac9565b9060243560443591606435906040516020810190848252856040820152836060820152606081526103ed608082610d0d565b51902091825f52600860205260ff60405f20541661051f5761048a61047961049392610417610f5b565b906040519060208201927faf2dfd3fe08723f490d203be627da2725f4ad38681e455221da2fc1a633bbb18845260018060a01b0316604083015288606083015289608083015260a082015260a0815261047160c082610d0d565b519020611641565b61048436898661103a565b90611593565b909291926115cd565b6001600160a01b036104a3610f5b565b166001600160a01b03821614801590610500575b6104dd57505061024c93505f52600860205260405f20600160ff19825416179055611301565b604051630b00088b60e11b81529182916104fc91889060048501610cd5565b0390fd5b506001600160a01b0381165f90815260046020526040902054156104b7565b836303da8f1360e31b5f5260045260245260445ffd5b346101695760403660031901126101695760043567ffffffffffffffff811161016957610566903690600401610b4e565b60243567ffffffffffffffff811161016957610586903690600401610b4e565b335f90815260046020526040902054909290156105bc578083036105ad5761024c9361116f565b63b4fa3fb360e01b5f5260045ffd5b63bf18af4360e01b5f523360045260245ffd5b34610169575f366003190112610169575f546040516001600160a01b039091168152602090f35b34610169575f366003190112610169576106956106327f000000000000000000000000000000000000000000000000000000000000000061139a565b61016561065e7f00000000000000000000000000000000000000000000000000000000000000006114c3565b6106a360405191610670602084610d0d565b5f83525f368137604051958695600f60f81b875260e0602088015260e0870190610b2a565b908582036040870152610b2a565b904660608501523060808501525f60a085015283820360c0850152610af7565b34610169575f36600319011261016957600154336001600160a01b039091160361073557600180546001600160a01b03199081169091555f805433928116831782556001600160a01b0316907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3005b63118cdaa760e01b5f523360045260245ffd5b34610169575f36600319011261016957610760611027565b600180546001600160a01b03199081169091555f80549182168155906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461016957604036600319011261016957335f90815260046020526040902054156105bc5761024c602435600435611301565b346101695760403660031901126101695760043567ffffffffffffffff81116101695780600401606060031983360301126101695760243567ffffffffffffffff811161016957610833903690600401610ac9565b60448401929160246108458585610f25565b96905001946108548685610f25565b919050036105ad576108f761048a6108ec61086e86610f71565b61047161087b8a89610f25565b61013b61088b8c8c959495610f25565b6108da60405196879560208701997fb06793f900067653959d9bc53299ebf6b5aa5cf5f6c1a463305891a3db695f3c8b5260018060a01b031660408801526080606088015260a087019161113b565b848103601f190160808601529161113b565b61048436868661103a565b6001600160a01b0361090885610f71565b166001600160a01b03821614801590610960575b6109435750505061093b61093361024c9483610f25565b939092610f25565b92909161116f565b6104fc90604051938493630b00088b60e11b855260048501610cd5565b506001600160a01b0381165f908152600460205260409020541561091c565b3461016957602036600319011261016957610165610151600435611080565b346101695760a0366003190112610169576109b7610a9d565b506024356001600160a01b0381168103610169576109e76020916109d9610ab3565b506084359060643590610d4f565b6040516001600160e01b03199091168152f35b346101695760403660031901126101695760243567ffffffffffffffff811161016957610a2b903690600401610ac9565b90610a4561048a610a3d36858561103a565b600435611593565b6001600160a01b0381165f908152600460205260409020541561094357604051630b135d3f60e11b8152602090f35b346101695760203660031901126101695761024c610a90610a9d565b610a98611027565b610bab565b600435906001600160a01b038216820361016957565b604435906001600160a01b038216820361016957565b9181601f840112156101695782359167ffffffffffffffff8311610169576020838186019501011161016957565b90602080835192838152019201905f5b818110610b145750505090565b8251845260209384019390920191600101610b07565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9181601f840112156101695782359167ffffffffffffffff8311610169576020808501948460051b01011161016957565b600554811015610b975760055f5260205f2001905f90565b634e487b7160e01b5f52603260045260245ffd5b6001600160a01b0381165f9081526004602052604090205415610cd2576001600160a01b03165f818152600460205260409020545f198101908111610cbe576005545f19810191908211610cbe57610c20610c08610c4493610b7f565b905460039190911b1c6001600160a01b031691610b7f565b81546001600160a01b0393841660039290921b91821b9390911b1916919091179055565b6005548015610caa577f3525e22824a8a7df2c9a6029941c824cf95b6447f1e13d5128fd3826d35afe8b916020915f1901610c7e81610b7f565b81549060018060a01b039060031b1b19169055600555805f52600482525f6040812055604051908152a1565b634e487b7160e01b5f52603160045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b50565b918060609160209396959660408652816040870152838601375f828286010152601f80199101168301019360018060a01b0316910152565b90601f8019910116810190811067ffffffffffffffff821117610d2f57604052565b634e487b7160e01b5f52604160045260245ffd5b8015610cbe575f190190565b9291907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633819003610f0f5750604080516001600160a01b0386166020820190815291810183905260608101849052610db4816080810161013b565b51902093845f52600760205260405f2054928315610efc5783805b610de65786630200745560e31b5f5260045260245ffd5b604051602081019088825282604082015260408152610e06606082610d0d565b519020805f5260066020524260405f20541015610e2d5750610e2790610d43565b80610dcf565b85610e7591610ea79596977feeb125dce1d8bff72304500b7a5fb59d2cc1fdd94698d12454917b26d6a9ae90999a94145f14610eb5575f5260066020525f6040812055610d43565b905f52600760205260405f205560405193849384604091949392606082019560018060a01b0316825260208201520152565b0390a1631a808f9160e01b90565b604051602081019085825283604082015260408152610ed5606082610d0d565b5190205f818152600660205260408082208054948352908220939093559081529055610d43565b85636a7a7c0b60e01b5f5260045260245ffd5b6302d9d9c960e31b5f523360045260245260445ffd5b903590601e1981360301821215610169570180359067ffffffffffffffff821161016957602001918160051b3603831361016957565b6004356001600160a01b03811681036101695790565b356001600160a01b03811681036101695790565b67ffffffffffffffff8111610d2f5760051b60200190565b6001600160a01b0381165f90815260046020526040902054610cd25760055468010000000000000000811015610d2f57816110057f47d1c22a25bb3a5d4e481b9b1e6944c2eade3181a0a20b495ed61d35b5323f2493610c2084600160209601600555610b7f565b6005549060018060a01b031690815f526004835260405f2055604051908152a1565b5f546001600160a01b0316330361073557565b92919267ffffffffffffffff8211610d2f5760405191611064601f8201601f191660200184610d0d565b829481845281830111610169578281602093845f960137010152565b805f52600760205260405f20549081156111295761109d82610f85565b916110ab6040519384610d0d565b808352601f196110ba82610f85565b01366020850137805b6110cc57505090565b6040516020810190838252826040820152604081526110ec606082610d0d565b5190205f52600660205260405f2054905f19810191818311610cbe578451831015610b975760206111239360051b86010152610d43565b806110c3565b636a7a7c0b60e01b5f5260045260245ffd5b81835290916001600160fb1b0383116101695760209260051b809284830137010190565b9190811015610b975760051b0190565b919392936040516312d4888560e01b8152602060048201526020818061119960248201878961113b565b03815f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af180156112f6576112bf575b505f5b85811061121f5750507f4f5e66e3a2d3cca9c3f07b4dce932f0035f527a89177c55267fce8a39a6bb33a92935061121a60405192839260208452602084019161113b565b0390a1565b8061122d600192888561115f565b35611239575b016111d6565b61124481888561115f565b355f52600760205260405f20548061125d575b50611233565b6112a19061126c838a8761115f565b35604051602081019182528260408201526040815261128c606082610d0d565b5190205f5260066020525f6040812055610d43565b6112ac82898661115f565b355f52600760205260405f20555f611257565b6020813d6020116112ee575b816112d860209383610d0d565b81010312610169575180151581146111d3575f80fd5b3d91506112cb565b6040513d5f823e3d90fd5b9042811061138457815f52600760205260405f20918254915f198314610cbe577f5ff03ecca156e50cd40af1660daac39e5ba1c930959671fbb0d3f5d660fb7815936001604094018091558351602081019184835285820152848152611368606082610d0d565b5190205f52600660205280835f205582519182526020820152a1565b63aa2fd92560e01b5f526004524260245260445ffd5b60ff81146113e05760ff811690601f82116113d157604051916113be604084610d0d565b6020808452838101919036833783525290565b632cd44ac360e21b5f5260045ffd5b506040515f6002548060011c91600182169182156114b9575b6020841083146114a55783855284929081156114865750600114611427575b61142492500382610d0d565b90565b5060025f90815290917f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace5b81831061146a57505090602061142492820101611418565b6020919350806001915483858801015201910190918392611452565b6020925061142494915060ff191682840152151560051b820101611418565b634e487b7160e01b5f52602260045260245ffd5b92607f16926113f9565b60ff81146114e75760ff811690601f82116113d157604051916113be604084610d0d565b506040515f6003548060011c9160018216918215611589575b6020841083146114a5578385528492908115611486575060011461152a5761142492500382610d0d565b5060035f90815290917fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b5b81831061156d57505090602061142492820101611418565b6020919350806001915483858801015201910190918392611555565b92607f1692611500565b81519190604183036115c3576115bc9250602082015190606060408401519301515f1a90611667565b9192909190565b50505f9160029190565b600481101561162d57806115df575050565b600181036115f65763f645eedf60e01b5f5260045ffd5b60028103611611575063fce698f760e01b5f5260045260245ffd5b60031461161b5750565b6335e2f38360e21b5f5260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b60429061164c6116e9565b906040519161190160f01b8352600283015260228201522090565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116116de579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa156112f6575f516001600160a01b038116156116d457905f905f90565b505f906001905f90565b5050505f9160039190565b307f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614806117dc575b15611744577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a081526117d660c082610d0d565b51902090565b507f0000000000000000000000000000000000000000000000000000000000000000461461171b56fea164736f6c634300081c000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80\x80`@R4a\x02PW`@\x81a\x1D\xAB\x808\x03\x80\x91a\0 \x82\x85a\x02zV[\x839\x81\x01\x03\x12a\x02PWa\0?` a\08\x83a\x02\xB1V[\x92\x01a\x02\xB1V[\x90`@Q\x91a\0O`@\x84a\x02zV[`\t\x83R` \x83\x01\x91h \xB667\xB1\xB0\xBA7\xB9`\xB9\x1B\x83R`@Q\x90a\0v`@\x83a\x02zV[`\x01\x82R`1`\xF8\x1B` \x83\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x93\x84\x15a\x02gW`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U_\x80T\x91\x82\x16\x87\x17\x81U\x96` \x96\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x89\x80\xA3a\0\xF1\x81a\x02\xC5V[a\x01 Ra\0\xFE\x84a\x04`V[a\x01@RQ\x90 \x91\x82`\xE0RQ\x90 \x80a\x01\0RF`\xA0R`@Q\x90\x84\x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x01f`\xC0\x82a\x02zV[Q\x90 `\x80R0`\xC0R\x80a\x01`R`d`@Q\x80\x94\x81\x93c*\x9CM\r`\xE0\x1B\x83R0`\x04\x84\x01R`@`$\x84\x01R\x81`D\x84\x01R`\x01\x80`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x02\\Wa\x02\x19W[`@Qa\x18\x12\x90\x81a\x05\x99\x829`\x80Q\x81a\x17\"\x01R`\xA0Q\x81a\x17\xDF\x01R`\xC0Q\x81a\x16\xEC\x01R`\xE0Q\x81a\x17q\x01Ra\x01\0Q\x81a\x17\x97\x01Ra\x01 Q\x81a\x06\x0E\x01Ra\x01@Q\x81a\x06:\x01Ra\x01`Q\x81\x81\x81a\x02\x8B\x01R\x81\x81a\rT\x01Ra\x11\x9E\x01R\xF3[` \x81=` \x11a\x02TW[\x81a\x022` \x93\x83a\x02zV[\x81\x01\x03\x12a\x02PWQ`\x01`\x01``\x1B\x03\x81\x16\x03a\x02PW_a\x01\xB0V[_\x80\xFD[=\x91Pa\x02%V[`@Q=_\x82>=\x90\xFD[c\x1EO\xBD\xF7`\xE0\x1B_R_`\x04R`$_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\x9DW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02PWV[\x90\x81Q` \x81\x10_\x14a\x03?WP\x90`\x1F\x81Q\x11a\x02\xFFW` \x81Q\x91\x01Q` \x82\x10a\x02\xF0W\x17\x90V[_\x19\x82` \x03`\x03\x1B\x1B\x16\x17\x90V[`D` \x91`@Q\x92\x83\x91c0Z'\xA9`\xE0\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD[`\x01`\x01`@\x1B\x03\x81\x11a\x02\x9DW`\x02T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x04VW[` \x82\x10\x14a\x04BW`\x1F\x81\x11a\x04\x0FW[P` \x92`\x1F\x82\x11`\x01\x14a\x03\xAEW\x92\x81\x92\x93_\x92a\x03\xA3W[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x02U`\xFF\x90V[\x01Q\x90P_\x80a\x03\x8AV[`\x1F\x19\x82\x16\x93`\x02_R\x80_ \x91_[\x86\x81\x10a\x03\xF7WP\x83`\x01\x95\x96\x10a\x03\xDFW[PPP\x81\x1B\x01`\x02U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x03\xD1V[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x03\xBEV[`\x02_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90`\x1F\x83\x01`\x05\x1C\x01[\x81\x81\x10a\x047WPa\x03pV[_\x81U`\x01\x01a\x04*V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x90`\x7F\x16\x90a\x03^V[\x90\x81Q` \x81\x10_\x14a\x04\x8BWP\x90`\x1F\x81Q\x11a\x02\xFFW` \x81Q\x91\x01Q` \x82\x10a\x02\xF0W\x17\x90V[`\x01`\x01`@\x1B\x03\x81\x11a\x02\x9DW`\x03T`\x01\x81\x81\x1C\x91\x16\x80\x15a\x05\x8EW[` \x82\x10\x14a\x04BW`\x1F\x81\x11a\x05[W[P` \x92`\x1F\x82\x11`\x01\x14a\x04\xFAW\x92\x81\x92\x93_\x92a\x04\xEFW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17`\x03U`\xFF\x90V[\x01Q\x90P_\x80a\x04\xD6V[`\x1F\x19\x82\x16\x93`\x03_R\x80_ \x91_[\x86\x81\x10a\x05CWP\x83`\x01\x95\x96\x10a\x05+W[PPP\x81\x1B\x01`\x03U`\xFF\x90V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x05\x1DV[\x91\x92` `\x01\x81\x92\x86\x85\x01Q\x81U\x01\x94\x01\x92\x01a\x05\nV[`\x03_R`\x1F` _ \x91\x01`\x05\x1C\x81\x01\x90`\x1F\x83\x01`\x05\x1C\x01[\x81\x81\x10a\x05\x83WPa\x04\xBCV[_\x81U`\x01\x01a\x05vV[\x90`\x7F\x16\x90a\x04\xAAV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0E1j\xB7\x14a\ntW\x80c\x16&\xBA~\x14a\t\xFAW\x80c\x1A\x80\x8F\x91\x14a\t\x9EW\x80c%B\x04\xC5\x14a\t\x7FW\x80c+\xCAD\x7F\x14a\x07\xDEW\x80c-\xF9uF\x14a\x07\xABW\x80cqP\x18\xA6\x14a\x07HW\x80cy\xBAP\x97\x14a\x06\xC3W\x80c\x84\xB0\x19n\x14a\x05\xF6W\x80c\x8D\xA5\xCB[\x14a\x05\xCFW\x80c\xC9\xD0\xFA\x86\x14a\x055W\x80c\xCF\xDECt\x14a\x03\x82W\x80c\xD4//5\x14a\x02\xBAW\x80c\xD6\x99kn\x14a\x02vW\x80c\xE3\x0C9x\x14a\x02NW\x80c\xEB\x12\xD6\x1E\x14a\x02#W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xB1W\x80c\xF7\x80\xC0\xD5\x14a\x01mWc\xFCy\x10\x1E\x14a\0\xEAW_\x80\xFD[4a\x01iW``6`\x03\x19\x01\x12a\x01iWa\x01ea\x01Qa\x01\ta\n\x9DV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01\x90\x81R`$5\x91\x83\x01\x91\x90\x91R`D5``\x83\x01R\x90a\x01I\x81`\x80\x81\x01[\x03`\x1F\x19\x81\x01\x83R\x82a\r\rV[Q\x90 a\x10\x80V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\n\xF7V[\x03\x90\xF3[_\x80\xFD[4a\x01iW` 6`\x03\x19\x01\x12a\x01iW` a\x01\xA7a\x01\x8Ba\n\x9DV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x04` R`@\x90 T\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x01iW` 6`\x03\x19\x01\x12a\x01iWa\x01\xCAa\n\x9DV[a\x01\xD2a\x10'V[`\x01\x80`\xA0\x1B\x03\x16\x80k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x01T\x16\x17`\x01U`\x01\x80`\xA0\x1B\x03_T\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0_\x80\xA3\0[4a\x01iW` 6`\x03\x19\x01\x12a\x01iWa\x02La\x02?a\n\x9DV[a\x02Ga\x10'V[a\x0F\x9DV[\0[4a\x01iW_6`\x03\x19\x01\x12a\x01iW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01iW_6`\x03\x19\x01\x12a\x01iW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01iW_6`\x03\x19\x01\x12a\x01iW`@Q\x80` `\x05T\x92\x83\x81R\x01\x80\x92`\x05_R\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x90_[\x81\x81\x10a\x03cWPPP\x81a\x03\x18\x91\x03\x82a\r\rV[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x03AWPPP\x03\x90\xF3[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x033V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x03\x02V[4a\x01iW6`\x03\x19\x01`\xA0\x81\x12a\x01iW`\x80\x13a\x01iW`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\x03\xBB\x906\x90`\x04\x01a\n\xC9V[\x90`$5`D5\x91`d5\x90`@Q` \x81\x01\x90\x84\x82R\x85`@\x82\x01R\x83``\x82\x01R``\x81Ra\x03\xED`\x80\x82a\r\rV[Q\x90 \x91\x82_R`\x08` R`\xFF`@_ T\x16a\x05\x1FWa\x04\x8Aa\x04ya\x04\x93\x92a\x04\x17a\x0F[V[\x90`@Q\x90` \x82\x01\x92\x7F\xAF-\xFD?\xE0\x87#\xF4\x90\xD2\x03\xBEb}\xA2r_J\xD3\x86\x81\xE4U\"\x1D\xA2\xFC\x1Ac;\xBB\x18\x84R`\x01\x80`\xA0\x1B\x03\x16`@\x83\x01R\x88``\x83\x01R\x89`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\x04q`\xC0\x82a\r\rV[Q\x90 a\x16AV[a\x04\x846\x89\x86a\x10:V[\x90a\x15\x93V[\x90\x92\x91\x92a\x15\xCDV[`\x01`\x01`\xA0\x1B\x03a\x04\xA3a\x0F[V[\x16`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80\x15\x90a\x05\0W[a\x04\xDDWPPa\x02L\x93P_R`\x08` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua\x13\x01V[`@Qc\x0B\0\x08\x8B`\xE1\x1B\x81R\x91\x82\x91a\x04\xFC\x91\x88\x90`\x04\x85\x01a\x0C\xD5V[\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\x04\xB7V[\x83c\x03\xDA\x8F\x13`\xE3\x1B_R`\x04R`$R`D_\xFD[4a\x01iW`@6`\x03\x19\x01\x12a\x01iW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\x05f\x906\x90`\x04\x01a\x0BNV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\x05\x86\x906\x90`\x04\x01a\x0BNV[3_\x90\x81R`\x04` R`@\x90 T\x90\x92\x90\x15a\x05\xBCW\x80\x83\x03a\x05\xADWa\x02L\x93a\x11oV[c\xB4\xFA?\xB3`\xE0\x1B_R`\x04_\xFD[c\xBF\x18\xAFC`\xE0\x1B_R3`\x04R`$_\xFD[4a\x01iW_6`\x03\x19\x01\x12a\x01iW_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01iW_6`\x03\x19\x01\x12a\x01iWa\x06\x95a\x062\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\x9AV[a\x01ea\x06^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14\xC3V[a\x06\xA3`@Q\x91a\x06p` \x84a\r\rV[_\x83R_6\x817`@Q\x95\x86\x95`\x0F`\xF8\x1B\x87R`\xE0` \x88\x01R`\xE0\x87\x01\x90a\x0B*V[\x90\x85\x82\x03`@\x87\x01Ra\x0B*V[\x90F``\x85\x01R0`\x80\x85\x01R_`\xA0\x85\x01R\x83\x82\x03`\xC0\x85\x01Ra\n\xF7V[4a\x01iW_6`\x03\x19\x01\x12a\x01iW`\x01T3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x075W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U_\x80T3\x92\x81\x16\x83\x17\x82U`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3\0[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD[4a\x01iW_6`\x03\x19\x01\x12a\x01iWa\x07`a\x10'V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U_\x80T\x91\x82\x16\x81U\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01iW`@6`\x03\x19\x01\x12a\x01iW3_\x90\x81R`\x04` R`@\x90 T\x15a\x05\xBCWa\x02L`$5`\x045a\x13\x01V[4a\x01iW`@6`\x03\x19\x01\x12a\x01iW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iW\x80`\x04\x01```\x03\x19\x836\x03\x01\x12a\x01iW`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\x083\x906\x90`\x04\x01a\n\xC9V[`D\x84\x01\x92\x91`$a\x08E\x85\x85a\x0F%V[\x96\x90P\x01\x94a\x08T\x86\x85a\x0F%V[\x91\x90P\x03a\x05\xADWa\x08\xF7a\x04\x8Aa\x08\xECa\x08n\x86a\x0FqV[a\x04qa\x08{\x8A\x89a\x0F%V[a\x01;a\x08\x8B\x8C\x8C\x95\x94\x95a\x0F%V[a\x08\xDA`@Q\x96\x87\x95` \x87\x01\x99\x7F\xB0g\x93\xF9\0\x06vS\x95\x9D\x9B\xC52\x99\xEB\xF6\xB5\xAA\\\xF5\xF6\xC1\xA4c0X\x91\xA3\xDBi_<\x8BR`\x01\x80`\xA0\x1B\x03\x16`@\x88\x01R`\x80``\x88\x01R`\xA0\x87\x01\x91a\x11;V[\x84\x81\x03`\x1F\x19\x01`\x80\x86\x01R\x91a\x11;V[a\x04\x846\x86\x86a\x10:V[`\x01`\x01`\xA0\x1B\x03a\t\x08\x85a\x0FqV[\x16`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80\x15\x90a\t`W[a\tCWPPPa\t;a\t3a\x02L\x94\x83a\x0F%V[\x93\x90\x92a\x0F%V[\x92\x90\x91a\x11oV[a\x04\xFC\x90`@Q\x93\x84\x93c\x0B\0\x08\x8B`\xE1\x1B\x85R`\x04\x85\x01a\x0C\xD5V[P`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\t\x1CV[4a\x01iW` 6`\x03\x19\x01\x12a\x01iWa\x01ea\x01Q`\x045a\x10\x80V[4a\x01iW`\xA06`\x03\x19\x01\x12a\x01iWa\t\xB7a\n\x9DV[P`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01iWa\t\xE7` \x91a\t\xD9a\n\xB3V[P`\x845\x90`d5\x90a\rOV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R\xF3[4a\x01iW`@6`\x03\x19\x01\x12a\x01iW`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\n+\x906\x90`\x04\x01a\n\xC9V[\x90a\nEa\x04\x8Aa\n=6\x85\x85a\x10:V[`\x045a\x15\x93V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\tCW`@Qc\x0B\x13]?`\xE1\x1B\x81R` \x90\xF3[4a\x01iW` 6`\x03\x19\x01\x12a\x01iWa\x02La\n\x90a\n\x9DV[a\n\x98a\x10'V[a\x0B\xABV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01iWV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01iWV[\x91\x81`\x1F\x84\x01\x12\x15a\x01iW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01iW` \x83\x81\x86\x01\x95\x01\x01\x11a\x01iWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\x14WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\x07V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\x01iW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01iW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01iWV[`\x05T\x81\x10\x15a\x0B\x97W`\x05_R` _ \x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\x0C\xD2W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R`\x04` R`@\x90 T_\x19\x81\x01\x90\x81\x11a\x0C\xBEW`\x05T_\x19\x81\x01\x91\x90\x82\x11a\x0C\xBEWa\x0C a\x0C\x08a\x0CD\x93a\x0B\x7FV[\x90T`\x03\x91\x90\x91\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16\x91a\x0B\x7FV[\x81T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x03\x92\x90\x92\x1B\x91\x82\x1B\x93\x90\x91\x1B\x19\x16\x91\x90\x91\x17\x90UV[`\x05T\x80\x15a\x0C\xAAW\x7F5%\xE2($\xA8\xA7\xDF,\x9A`)\x94\x1C\x82L\xF9[dG\xF1\xE1=Q(\xFD8&\xD3Z\xFE\x8B\x91` \x91_\x19\x01a\x0C~\x81a\x0B\x7FV[\x81T\x90`\x01\x80`\xA0\x1B\x03\x90`\x03\x1B\x1B\x19\x16\x90U`\x05U\x80_R`\x04\x82R_`@\x81 U`@Q\x90\x81R\xA1V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[PV[\x91\x80``\x91` \x93\x96\x95\x96`@\x86R\x81`@\x87\x01R\x83\x86\x017_\x82\x82\x86\x01\x01R`\x1F\x80\x19\x91\x01\x16\x83\x01\x01\x93`\x01\x80`\xA0\x1B\x03\x16\x91\x01RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r/W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x15a\x0C\xBEW_\x19\x01\x90V[\x92\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x81\x90\x03a\x0F\x0FWP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x16` \x82\x01\x90\x81R\x91\x81\x01\x83\x90R``\x81\x01\x84\x90Ra\r\xB4\x81`\x80\x81\x01a\x01;V[Q\x90 \x93\x84_R`\x07` R`@_ T\x92\x83\x15a\x0E\xFCW\x83\x80[a\r\xE6W\x86c\x02\0tU`\xE3\x1B_R`\x04R`$_\xFD[`@Q` \x81\x01\x90\x88\x82R\x82`@\x82\x01R`@\x81Ra\x0E\x06``\x82a\r\rV[Q\x90 \x80_R`\x06` RB`@_ T\x10\x15a\x0E-WPa\x0E'\x90a\rCV[\x80a\r\xCFV[\x85a\x0Eu\x91a\x0E\xA7\x95\x96\x97\x7F\xEE\xB1%\xDC\xE1\xD8\xBF\xF7#\x04P\x0Bz_\xB5\x9D,\xC1\xFD\xD9F\x98\xD1$T\x91{&\xD6\xA9\xAE\x90\x99\x9A\x94\x14_\x14a\x0E\xB5W_R`\x06` R_`@\x81 Ua\rCV[\x90_R`\x07` R`@_ U`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x01RV[\x03\x90\xA1c\x1A\x80\x8F\x91`\xE0\x1B\x90V[`@Q` \x81\x01\x90\x85\x82R\x83`@\x82\x01R`@\x81Ra\x0E\xD5``\x82a\r\rV[Q\x90 _\x81\x81R`\x06` R`@\x80\x82 \x80T\x94\x83R\x90\x82 \x93\x90\x93U\x90\x81R\x90Ua\rCV[\x85cjz|\x0B`\xE0\x1B_R`\x04R`$_\xFD[c\x02\xD9\xD9\xC9`\xE3\x1B_R3`\x04R`$R`D_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01iW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01iW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x01iWV[`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01iW\x90V[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01iW\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r/W`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 Ta\x0C\xD2W`\x05Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\r/W\x81a\x10\x05\x7FG\xD1\xC2*%\xBB:]NH\x1B\x9B\x1EiD\xC2\xEA\xDE1\x81\xA0\xA2\x0BI^\xD6\x1D5\xB52?$\x93a\x0C \x84`\x01` \x96\x01`\x05Ua\x0B\x7FV[`\x05T\x90`\x01\x80`\xA0\x1B\x03\x16\x90\x81_R`\x04\x83R`@_ U`@Q\x90\x81R\xA1V[_T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x075WV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\r/W`@Q\x91a\x10d`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\r\rV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01iW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x80_R`\x07` R`@_ T\x90\x81\x15a\x11)Wa\x10\x9D\x82a\x0F\x85V[\x91a\x10\xAB`@Q\x93\x84a\r\rV[\x80\x83R`\x1F\x19a\x10\xBA\x82a\x0F\x85V[\x016` \x85\x017\x80[a\x10\xCCWPP\x90V[`@Q` \x81\x01\x90\x83\x82R\x82`@\x82\x01R`@\x81Ra\x10\xEC``\x82a\r\rV[Q\x90 _R`\x06` R`@_ T\x90_\x19\x81\x01\x91\x81\x83\x11a\x0C\xBEW\x84Q\x83\x10\x15a\x0B\x97W` a\x11#\x93`\x05\x1B\x86\x01\x01Ra\rCV[\x80a\x10\xC3V[cjz|\x0B`\xE0\x1B_R`\x04R`$_\xFD[\x81\x83R\x90\x91`\x01`\x01`\xFB\x1B\x03\x83\x11a\x01iW` \x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x91\x90\x81\x10\x15a\x0B\x97W`\x05\x1B\x01\x90V[\x91\x93\x92\x93`@Qc\x12\xD4\x88\x85`\xE0\x1B\x81R` `\x04\x82\x01R` \x81\x80a\x11\x99`$\x82\x01\x87\x89a\x11;V[\x03\x81_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x12\xF6Wa\x12\xBFW[P_[\x85\x81\x10a\x12\x1FWPP\x7FO^f\xE3\xA2\xD3\xCC\xA9\xC3\xF0{M\xCE\x93/\x005\xF5'\xA8\x91w\xC5Rg\xFC\xE8\xA3\x9Ak\xB3:\x92\x93Pa\x12\x1A`@Q\x92\x83\x92` \x84R` \x84\x01\x91a\x11;V[\x03\x90\xA1V[\x80a\x12-`\x01\x92\x88\x85a\x11_V[5a\x129W[\x01a\x11\xD6V[a\x12D\x81\x88\x85a\x11_V[5_R`\x07` R`@_ T\x80a\x12]W[Pa\x123V[a\x12\xA1\x90a\x12l\x83\x8A\x87a\x11_V[5`@Q` \x81\x01\x91\x82R\x82`@\x82\x01R`@\x81Ra\x12\x8C``\x82a\r\rV[Q\x90 _R`\x06` R_`@\x81 Ua\rCV[a\x12\xAC\x82\x89\x86a\x11_V[5_R`\x07` R`@_ U_a\x12WV[` \x81=` \x11a\x12\xEEW[\x81a\x12\xD8` \x93\x83a\r\rV[\x81\x01\x03\x12a\x01iWQ\x80\x15\x15\x81\x14a\x11\xD3W_\x80\xFD[=\x91Pa\x12\xCBV[`@Q=_\x82>=\x90\xFD[\x90B\x81\x10a\x13\x84W\x81_R`\x07` R`@_ \x91\x82T\x91_\x19\x83\x14a\x0C\xBEW\x7F_\xF0>\xCC\xA1V\xE5\x0C\xD4\n\xF1f\r\xAA\xC3\x9E[\xA1\xC90\x95\x96q\xFB\xB0\xD3\xF5\xD6`\xFBx\x15\x93`\x01`@\x94\x01\x80\x91U\x83Q` \x81\x01\x91\x84\x83R\x85\x82\x01R\x84\x81Ra\x13h``\x82a\r\rV[Q\x90 _R`\x06` R\x80\x83_ U\x82Q\x91\x82R` \x82\x01R\xA1V[c\xAA/\xD9%`\xE0\x1B_R`\x04RB`$R`D_\xFD[`\xFF\x81\x14a\x13\xE0W`\xFF\x81\x16\x90`\x1F\x82\x11a\x13\xD1W`@Q\x91a\x13\xBE`@\x84a\r\rV[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[c,\xD4J\xC3`\xE2\x1B_R`\x04_\xFD[P`@Q_`\x02T\x80`\x01\x1C\x91`\x01\x82\x16\x91\x82\x15a\x14\xB9W[` \x84\x10\x83\x14a\x14\xA5W\x83\x85R\x84\x92\x90\x81\x15a\x14\x86WP`\x01\x14a\x14'W[a\x14$\x92P\x03\x82a\r\rV[\x90V[P`\x02_\x90\x81R\x90\x91\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE[\x81\x83\x10a\x14jWPP\x90` a\x14$\x92\x82\x01\x01a\x14\x18V[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x14RV[` \x92Pa\x14$\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01a\x14\x18V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x92`\x7F\x16\x92a\x13\xF9V[`\xFF\x81\x14a\x14\xE7W`\xFF\x81\x16\x90`\x1F\x82\x11a\x13\xD1W`@Q\x91a\x13\xBE`@\x84a\r\rV[P`@Q_`\x03T\x80`\x01\x1C\x91`\x01\x82\x16\x91\x82\x15a\x15\x89W[` \x84\x10\x83\x14a\x14\xA5W\x83\x85R\x84\x92\x90\x81\x15a\x14\x86WP`\x01\x14a\x15*Wa\x14$\x92P\x03\x82a\r\rV[P`\x03_\x90\x81R\x90\x91\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[[\x81\x83\x10a\x15mWPP\x90` a\x14$\x92\x82\x01\x01a\x14\x18V[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x15UV[\x92`\x7F\x16\x92a\x15\0V[\x81Q\x91\x90`A\x83\x03a\x15\xC3Wa\x15\xBC\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\x16gV[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15a\x16-W\x80a\x15\xDFWPPV[`\x01\x81\x03a\x15\xF6Wc\xF6E\xEE\xDF`\xE0\x1B_R`\x04_\xFD[`\x02\x81\x03a\x16\x11WPc\xFC\xE6\x98\xF7`\xE0\x1B_R`\x04R`$_\xFD[`\x03\x14a\x16\x1BWPV[c5\xE2\xF3\x83`\xE2\x1B_R`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`B\x90a\x16La\x16\xE9V[\x90`@Q\x91a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a\x16\xDEW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x12\xF6W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x16\xD4W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x17\xDCW[\x15a\x17DW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x17\xD6`\xC0\x82a\r\rV[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x17\x1BV\xFE\xA1dsolcC\0\x08\x1C\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c80630e316ab714610a745780631626ba7e146109fa5780631a808f911461099e578063254204c51461097f5780632bca447f146107de5780632df97546146107ab578063715018a61461074857806379ba5097146106c357806384b0196e146105f65780638da5cb5b146105cf578063c9d0fa8614610535578063cfde437414610382578063d42f2f35146102ba578063d6996b6e14610276578063e30c39781461024e578063eb12d61e14610223578063f2fde38b146101b1578063f780c0d51461016d5763fc79101e146100ea575f80fd5b3461016957606036600319011261016957610165610151610109610a9d565b604080516001600160a01b03909216602083019081526024359183019190915260443560608301529061014981608081015b03601f198101835282610d0d565b519020611080565b604051918291602083526020830190610af7565b0390f35b5f80fd5b346101695760203660031901126101695760206101a761018b610a9d565b6001600160a01b03165f90815260046020526040902054151590565b6040519015158152f35b34610169576020366003190112610169576101ca610a9d565b6101d2611027565b60018060a01b0316806bffffffffffffffffffffffff60a01b600154161760015560018060a01b035f54167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e227005f80a3005b346101695760203660031901126101695761024c61023f610a9d565b610247611027565b610f9d565b005b34610169575f366003190112610169576001546040516001600160a01b039091168152602090f35b34610169575f366003190112610169576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610169575f366003190112610169576040518060206005549283815201809260055f527f036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db0905f5b8181106103635750505081610318910382610d0d565b604051918291602083019060208452518091526040830191905f5b818110610341575050500390f35b82516001600160a01b0316845285945060209384019390920191600101610333565b82546001600160a01b0316845260209093019260019283019201610302565b3461016957366003190160a08112610169576080136101695760843567ffffffffffffffff8111610169576103bb903690600401610ac9565b9060243560443591606435906040516020810190848252856040820152836060820152606081526103ed608082610d0d565b51902091825f52600860205260ff60405f20541661051f5761048a61047961049392610417610f5b565b906040519060208201927faf2dfd3fe08723f490d203be627da2725f4ad38681e455221da2fc1a633bbb18845260018060a01b0316604083015288606083015289608083015260a082015260a0815261047160c082610d0d565b519020611641565b61048436898661103a565b90611593565b909291926115cd565b6001600160a01b036104a3610f5b565b166001600160a01b03821614801590610500575b6104dd57505061024c93505f52600860205260405f20600160ff19825416179055611301565b604051630b00088b60e11b81529182916104fc91889060048501610cd5565b0390fd5b506001600160a01b0381165f90815260046020526040902054156104b7565b836303da8f1360e31b5f5260045260245260445ffd5b346101695760403660031901126101695760043567ffffffffffffffff811161016957610566903690600401610b4e565b60243567ffffffffffffffff811161016957610586903690600401610b4e565b335f90815260046020526040902054909290156105bc578083036105ad5761024c9361116f565b63b4fa3fb360e01b5f5260045ffd5b63bf18af4360e01b5f523360045260245ffd5b34610169575f366003190112610169575f546040516001600160a01b039091168152602090f35b34610169575f366003190112610169576106956106327f000000000000000000000000000000000000000000000000000000000000000061139a565b61016561065e7f00000000000000000000000000000000000000000000000000000000000000006114c3565b6106a360405191610670602084610d0d565b5f83525f368137604051958695600f60f81b875260e0602088015260e0870190610b2a565b908582036040870152610b2a565b904660608501523060808501525f60a085015283820360c0850152610af7565b34610169575f36600319011261016957600154336001600160a01b039091160361073557600180546001600160a01b03199081169091555f805433928116831782556001600160a01b0316907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09080a3005b63118cdaa760e01b5f523360045260245ffd5b34610169575f36600319011261016957610760611027565b600180546001600160a01b03199081169091555f80549182168155906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461016957604036600319011261016957335f90815260046020526040902054156105bc5761024c602435600435611301565b346101695760403660031901126101695760043567ffffffffffffffff81116101695780600401606060031983360301126101695760243567ffffffffffffffff811161016957610833903690600401610ac9565b60448401929160246108458585610f25565b96905001946108548685610f25565b919050036105ad576108f761048a6108ec61086e86610f71565b61047161087b8a89610f25565b61013b61088b8c8c959495610f25565b6108da60405196879560208701997fb06793f900067653959d9bc53299ebf6b5aa5cf5f6c1a463305891a3db695f3c8b5260018060a01b031660408801526080606088015260a087019161113b565b848103601f190160808601529161113b565b61048436868661103a565b6001600160a01b0361090885610f71565b166001600160a01b03821614801590610960575b6109435750505061093b61093361024c9483610f25565b939092610f25565b92909161116f565b6104fc90604051938493630b00088b60e11b855260048501610cd5565b506001600160a01b0381165f908152600460205260409020541561091c565b3461016957602036600319011261016957610165610151600435611080565b346101695760a0366003190112610169576109b7610a9d565b506024356001600160a01b0381168103610169576109e76020916109d9610ab3565b506084359060643590610d4f565b6040516001600160e01b03199091168152f35b346101695760403660031901126101695760243567ffffffffffffffff811161016957610a2b903690600401610ac9565b90610a4561048a610a3d36858561103a565b600435611593565b6001600160a01b0381165f908152600460205260409020541561094357604051630b135d3f60e11b8152602090f35b346101695760203660031901126101695761024c610a90610a9d565b610a98611027565b610bab565b600435906001600160a01b038216820361016957565b604435906001600160a01b038216820361016957565b9181601f840112156101695782359167ffffffffffffffff8311610169576020838186019501011161016957565b90602080835192838152019201905f5b818110610b145750505090565b8251845260209384019390920191600101610b07565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b9181601f840112156101695782359167ffffffffffffffff8311610169576020808501948460051b01011161016957565b600554811015610b975760055f5260205f2001905f90565b634e487b7160e01b5f52603260045260245ffd5b6001600160a01b0381165f9081526004602052604090205415610cd2576001600160a01b03165f818152600460205260409020545f198101908111610cbe576005545f19810191908211610cbe57610c20610c08610c4493610b7f565b905460039190911b1c6001600160a01b031691610b7f565b81546001600160a01b0393841660039290921b91821b9390911b1916919091179055565b6005548015610caa577f3525e22824a8a7df2c9a6029941c824cf95b6447f1e13d5128fd3826d35afe8b916020915f1901610c7e81610b7f565b81549060018060a01b039060031b1b19169055600555805f52600482525f6040812055604051908152a1565b634e487b7160e01b5f52603160045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b50565b918060609160209396959660408652816040870152838601375f828286010152601f80199101168301019360018060a01b0316910152565b90601f8019910116810190811067ffffffffffffffff821117610d2f57604052565b634e487b7160e01b5f52604160045260245ffd5b8015610cbe575f190190565b9291907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633819003610f0f5750604080516001600160a01b0386166020820190815291810183905260608101849052610db4816080810161013b565b51902093845f52600760205260405f2054928315610efc5783805b610de65786630200745560e31b5f5260045260245ffd5b604051602081019088825282604082015260408152610e06606082610d0d565b519020805f5260066020524260405f20541015610e2d5750610e2790610d43565b80610dcf565b85610e7591610ea79596977feeb125dce1d8bff72304500b7a5fb59d2cc1fdd94698d12454917b26d6a9ae90999a94145f14610eb5575f5260066020525f6040812055610d43565b905f52600760205260405f205560405193849384604091949392606082019560018060a01b0316825260208201520152565b0390a1631a808f9160e01b90565b604051602081019085825283604082015260408152610ed5606082610d0d565b5190205f818152600660205260408082208054948352908220939093559081529055610d43565b85636a7a7c0b60e01b5f5260045260245ffd5b6302d9d9c960e31b5f523360045260245260445ffd5b903590601e1981360301821215610169570180359067ffffffffffffffff821161016957602001918160051b3603831361016957565b6004356001600160a01b03811681036101695790565b356001600160a01b03811681036101695790565b67ffffffffffffffff8111610d2f5760051b60200190565b6001600160a01b0381165f90815260046020526040902054610cd25760055468010000000000000000811015610d2f57816110057f47d1c22a25bb3a5d4e481b9b1e6944c2eade3181a0a20b495ed61d35b5323f2493610c2084600160209601600555610b7f565b6005549060018060a01b031690815f526004835260405f2055604051908152a1565b5f546001600160a01b0316330361073557565b92919267ffffffffffffffff8211610d2f5760405191611064601f8201601f191660200184610d0d565b829481845281830111610169578281602093845f960137010152565b805f52600760205260405f20549081156111295761109d82610f85565b916110ab6040519384610d0d565b808352601f196110ba82610f85565b01366020850137805b6110cc57505090565b6040516020810190838252826040820152604081526110ec606082610d0d565b5190205f52600660205260405f2054905f19810191818311610cbe578451831015610b975760206111239360051b86010152610d43565b806110c3565b636a7a7c0b60e01b5f5260045260245ffd5b81835290916001600160fb1b0383116101695760209260051b809284830137010190565b9190811015610b975760051b0190565b919392936040516312d4888560e01b8152602060048201526020818061119960248201878961113b565b03815f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af180156112f6576112bf575b505f5b85811061121f5750507f4f5e66e3a2d3cca9c3f07b4dce932f0035f527a89177c55267fce8a39a6bb33a92935061121a60405192839260208452602084019161113b565b0390a1565b8061122d600192888561115f565b35611239575b016111d6565b61124481888561115f565b355f52600760205260405f20548061125d575b50611233565b6112a19061126c838a8761115f565b35604051602081019182528260408201526040815261128c606082610d0d565b5190205f5260066020525f6040812055610d43565b6112ac82898661115f565b355f52600760205260405f20555f611257565b6020813d6020116112ee575b816112d860209383610d0d565b81010312610169575180151581146111d3575f80fd5b3d91506112cb565b6040513d5f823e3d90fd5b9042811061138457815f52600760205260405f20918254915f198314610cbe577f5ff03ecca156e50cd40af1660daac39e5ba1c930959671fbb0d3f5d660fb7815936001604094018091558351602081019184835285820152848152611368606082610d0d565b5190205f52600660205280835f205582519182526020820152a1565b63aa2fd92560e01b5f526004524260245260445ffd5b60ff81146113e05760ff811690601f82116113d157604051916113be604084610d0d565b6020808452838101919036833783525290565b632cd44ac360e21b5f5260045ffd5b506040515f6002548060011c91600182169182156114b9575b6020841083146114a55783855284929081156114865750600114611427575b61142492500382610d0d565b90565b5060025f90815290917f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace5b81831061146a57505090602061142492820101611418565b6020919350806001915483858801015201910190918392611452565b6020925061142494915060ff191682840152151560051b820101611418565b634e487b7160e01b5f52602260045260245ffd5b92607f16926113f9565b60ff81146114e75760ff811690601f82116113d157604051916113be604084610d0d565b506040515f6003548060011c9160018216918215611589575b6020841083146114a5578385528492908115611486575060011461152a5761142492500382610d0d565b5060035f90815290917fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b5b81831061156d57505090602061142492820101611418565b6020919350806001915483858801015201910190918392611555565b92607f1692611500565b81519190604183036115c3576115bc9250602082015190606060408401519301515f1a90611667565b9192909190565b50505f9160029190565b600481101561162d57806115df575050565b600181036115f65763f645eedf60e01b5f5260045ffd5b60028103611611575063fce698f760e01b5f5260045260245ffd5b60031461161b5750565b6335e2f38360e21b5f5260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b60429061164c6116e9565b906040519161190160f01b8352600283015260228201522090565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084116116de579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa156112f6575f516001600160a01b038116156116d457905f905f90565b505f906001905f90565b5050505f9160039190565b307f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031614806117dc575b15611744577f000000000000000000000000000000000000000000000000000000000000000090565b60405160208101907f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f82527f000000000000000000000000000000000000000000000000000000000000000060408201527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a082015260a081526117d660c082610d0d565b51902090565b507f0000000000000000000000000000000000000000000000000000000000000000461461171b56fea164736f6c634300081c000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0E1j\xB7\x14a\ntW\x80c\x16&\xBA~\x14a\t\xFAW\x80c\x1A\x80\x8F\x91\x14a\t\x9EW\x80c%B\x04\xC5\x14a\t\x7FW\x80c+\xCAD\x7F\x14a\x07\xDEW\x80c-\xF9uF\x14a\x07\xABW\x80cqP\x18\xA6\x14a\x07HW\x80cy\xBAP\x97\x14a\x06\xC3W\x80c\x84\xB0\x19n\x14a\x05\xF6W\x80c\x8D\xA5\xCB[\x14a\x05\xCFW\x80c\xC9\xD0\xFA\x86\x14a\x055W\x80c\xCF\xDECt\x14a\x03\x82W\x80c\xD4//5\x14a\x02\xBAW\x80c\xD6\x99kn\x14a\x02vW\x80c\xE3\x0C9x\x14a\x02NW\x80c\xEB\x12\xD6\x1E\x14a\x02#W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xB1W\x80c\xF7\x80\xC0\xD5\x14a\x01mWc\xFCy\x10\x1E\x14a\0\xEAW_\x80\xFD[4a\x01iW``6`\x03\x19\x01\x12a\x01iWa\x01ea\x01Qa\x01\ta\n\x9DV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01\x90\x81R`$5\x91\x83\x01\x91\x90\x91R`D5``\x83\x01R\x90a\x01I\x81`\x80\x81\x01[\x03`\x1F\x19\x81\x01\x83R\x82a\r\rV[Q\x90 a\x10\x80V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\n\xF7V[\x03\x90\xF3[_\x80\xFD[4a\x01iW` 6`\x03\x19\x01\x12a\x01iW` a\x01\xA7a\x01\x8Ba\n\x9DV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x04` R`@\x90 T\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[4a\x01iW` 6`\x03\x19\x01\x12a\x01iWa\x01\xCAa\n\x9DV[a\x01\xD2a\x10'V[`\x01\x80`\xA0\x1B\x03\x16\x80k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`\x01T\x16\x17`\x01U`\x01\x80`\xA0\x1B\x03_T\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0_\x80\xA3\0[4a\x01iW` 6`\x03\x19\x01\x12a\x01iWa\x02La\x02?a\n\x9DV[a\x02Ga\x10'V[a\x0F\x9DV[\0[4a\x01iW_6`\x03\x19\x01\x12a\x01iW`\x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01iW_6`\x03\x19\x01\x12a\x01iW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01iW_6`\x03\x19\x01\x12a\x01iW`@Q\x80` `\x05T\x92\x83\x81R\x01\x80\x92`\x05_R\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x90_[\x81\x81\x10a\x03cWPPP\x81a\x03\x18\x91\x03\x82a\r\rV[`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x03AWPPP\x03\x90\xF3[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x033V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x03\x02V[4a\x01iW6`\x03\x19\x01`\xA0\x81\x12a\x01iW`\x80\x13a\x01iW`\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\x03\xBB\x906\x90`\x04\x01a\n\xC9V[\x90`$5`D5\x91`d5\x90`@Q` \x81\x01\x90\x84\x82R\x85`@\x82\x01R\x83``\x82\x01R``\x81Ra\x03\xED`\x80\x82a\r\rV[Q\x90 \x91\x82_R`\x08` R`\xFF`@_ T\x16a\x05\x1FWa\x04\x8Aa\x04ya\x04\x93\x92a\x04\x17a\x0F[V[\x90`@Q\x90` \x82\x01\x92\x7F\xAF-\xFD?\xE0\x87#\xF4\x90\xD2\x03\xBEb}\xA2r_J\xD3\x86\x81\xE4U\"\x1D\xA2\xFC\x1Ac;\xBB\x18\x84R`\x01\x80`\xA0\x1B\x03\x16`@\x83\x01R\x88``\x83\x01R\x89`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\x04q`\xC0\x82a\r\rV[Q\x90 a\x16AV[a\x04\x846\x89\x86a\x10:V[\x90a\x15\x93V[\x90\x92\x91\x92a\x15\xCDV[`\x01`\x01`\xA0\x1B\x03a\x04\xA3a\x0F[V[\x16`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80\x15\x90a\x05\0W[a\x04\xDDWPPa\x02L\x93P_R`\x08` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua\x13\x01V[`@Qc\x0B\0\x08\x8B`\xE1\x1B\x81R\x91\x82\x91a\x04\xFC\x91\x88\x90`\x04\x85\x01a\x0C\xD5V[\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\x04\xB7V[\x83c\x03\xDA\x8F\x13`\xE3\x1B_R`\x04R`$R`D_\xFD[4a\x01iW`@6`\x03\x19\x01\x12a\x01iW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\x05f\x906\x90`\x04\x01a\x0BNV[`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\x05\x86\x906\x90`\x04\x01a\x0BNV[3_\x90\x81R`\x04` R`@\x90 T\x90\x92\x90\x15a\x05\xBCW\x80\x83\x03a\x05\xADWa\x02L\x93a\x11oV[c\xB4\xFA?\xB3`\xE0\x1B_R`\x04_\xFD[c\xBF\x18\xAFC`\xE0\x1B_R3`\x04R`$_\xFD[4a\x01iW_6`\x03\x19\x01\x12a\x01iW_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01iW_6`\x03\x19\x01\x12a\x01iWa\x06\x95a\x062\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\x9AV[a\x01ea\x06^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x14\xC3V[a\x06\xA3`@Q\x91a\x06p` \x84a\r\rV[_\x83R_6\x817`@Q\x95\x86\x95`\x0F`\xF8\x1B\x87R`\xE0` \x88\x01R`\xE0\x87\x01\x90a\x0B*V[\x90\x85\x82\x03`@\x87\x01Ra\x0B*V[\x90F``\x85\x01R0`\x80\x85\x01R_`\xA0\x85\x01R\x83\x82\x03`\xC0\x85\x01Ra\n\xF7V[4a\x01iW_6`\x03\x19\x01\x12a\x01iW`\x01T3`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x075W`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U_\x80T3\x92\x81\x16\x83\x17\x82U`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x80\xA3\0[c\x11\x8C\xDA\xA7`\xE0\x1B_R3`\x04R`$_\xFD[4a\x01iW_6`\x03\x19\x01\x12a\x01iWa\x07`a\x10'V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U_\x80T\x91\x82\x16\x81U\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01iW`@6`\x03\x19\x01\x12a\x01iW3_\x90\x81R`\x04` R`@\x90 T\x15a\x05\xBCWa\x02L`$5`\x045a\x13\x01V[4a\x01iW`@6`\x03\x19\x01\x12a\x01iW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iW\x80`\x04\x01```\x03\x19\x836\x03\x01\x12a\x01iW`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\x083\x906\x90`\x04\x01a\n\xC9V[`D\x84\x01\x92\x91`$a\x08E\x85\x85a\x0F%V[\x96\x90P\x01\x94a\x08T\x86\x85a\x0F%V[\x91\x90P\x03a\x05\xADWa\x08\xF7a\x04\x8Aa\x08\xECa\x08n\x86a\x0FqV[a\x04qa\x08{\x8A\x89a\x0F%V[a\x01;a\x08\x8B\x8C\x8C\x95\x94\x95a\x0F%V[a\x08\xDA`@Q\x96\x87\x95` \x87\x01\x99\x7F\xB0g\x93\xF9\0\x06vS\x95\x9D\x9B\xC52\x99\xEB\xF6\xB5\xAA\\\xF5\xF6\xC1\xA4c0X\x91\xA3\xDBi_<\x8BR`\x01\x80`\xA0\x1B\x03\x16`@\x88\x01R`\x80``\x88\x01R`\xA0\x87\x01\x91a\x11;V[\x84\x81\x03`\x1F\x19\x01`\x80\x86\x01R\x91a\x11;V[a\x04\x846\x86\x86a\x10:V[`\x01`\x01`\xA0\x1B\x03a\t\x08\x85a\x0FqV[\x16`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80\x15\x90a\t`W[a\tCWPPPa\t;a\t3a\x02L\x94\x83a\x0F%V[\x93\x90\x92a\x0F%V[\x92\x90\x91a\x11oV[a\x04\xFC\x90`@Q\x93\x84\x93c\x0B\0\x08\x8B`\xE1\x1B\x85R`\x04\x85\x01a\x0C\xD5V[P`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\t\x1CV[4a\x01iW` 6`\x03\x19\x01\x12a\x01iWa\x01ea\x01Q`\x045a\x10\x80V[4a\x01iW`\xA06`\x03\x19\x01\x12a\x01iWa\t\xB7a\n\x9DV[P`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01iWa\t\xE7` \x91a\t\xD9a\n\xB3V[P`\x845\x90`d5\x90a\rOV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R\xF3[4a\x01iW`@6`\x03\x19\x01\x12a\x01iW`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01iWa\n+\x906\x90`\x04\x01a\n\xC9V[\x90a\nEa\x04\x8Aa\n=6\x85\x85a\x10:V[`\x045a\x15\x93V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\tCW`@Qc\x0B\x13]?`\xE1\x1B\x81R` \x90\xF3[4a\x01iW` 6`\x03\x19\x01\x12a\x01iWa\x02La\n\x90a\n\x9DV[a\n\x98a\x10'V[a\x0B\xABV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01iWV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01iWV[\x91\x81`\x1F\x84\x01\x12\x15a\x01iW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01iW` \x83\x81\x86\x01\x95\x01\x01\x11a\x01iWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\x14WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\x07V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\x01iW\x825\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01iW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01iWV[`\x05T\x81\x10\x15a\x0B\x97W`\x05_R` _ \x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 T\x15a\x0C\xD2W`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R`\x04` R`@\x90 T_\x19\x81\x01\x90\x81\x11a\x0C\xBEW`\x05T_\x19\x81\x01\x91\x90\x82\x11a\x0C\xBEWa\x0C a\x0C\x08a\x0CD\x93a\x0B\x7FV[\x90T`\x03\x91\x90\x91\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16\x91a\x0B\x7FV[\x81T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x03\x92\x90\x92\x1B\x91\x82\x1B\x93\x90\x91\x1B\x19\x16\x91\x90\x91\x17\x90UV[`\x05T\x80\x15a\x0C\xAAW\x7F5%\xE2($\xA8\xA7\xDF,\x9A`)\x94\x1C\x82L\xF9[dG\xF1\xE1=Q(\xFD8&\xD3Z\xFE\x8B\x91` \x91_\x19\x01a\x0C~\x81a\x0B\x7FV[\x81T\x90`\x01\x80`\xA0\x1B\x03\x90`\x03\x1B\x1B\x19\x16\x90U`\x05U\x80_R`\x04\x82R_`@\x81 U`@Q\x90\x81R\xA1V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[PV[\x91\x80``\x91` \x93\x96\x95\x96`@\x86R\x81`@\x87\x01R\x83\x86\x017_\x82\x82\x86\x01\x01R`\x1F\x80\x19\x91\x01\x16\x83\x01\x01\x93`\x01\x80`\xA0\x1B\x03\x16\x91\x01RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\r/W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x80\x15a\x0C\xBEW_\x19\x01\x90V[\x92\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x81\x90\x03a\x0F\x0FWP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x16` \x82\x01\x90\x81R\x91\x81\x01\x83\x90R``\x81\x01\x84\x90Ra\r\xB4\x81`\x80\x81\x01a\x01;V[Q\x90 \x93\x84_R`\x07` R`@_ T\x92\x83\x15a\x0E\xFCW\x83\x80[a\r\xE6W\x86c\x02\0tU`\xE3\x1B_R`\x04R`$_\xFD[`@Q` \x81\x01\x90\x88\x82R\x82`@\x82\x01R`@\x81Ra\x0E\x06``\x82a\r\rV[Q\x90 \x80_R`\x06` RB`@_ T\x10\x15a\x0E-WPa\x0E'\x90a\rCV[\x80a\r\xCFV[\x85a\x0Eu\x91a\x0E\xA7\x95\x96\x97\x7F\xEE\xB1%\xDC\xE1\xD8\xBF\xF7#\x04P\x0Bz_\xB5\x9D,\xC1\xFD\xD9F\x98\xD1$T\x91{&\xD6\xA9\xAE\x90\x99\x9A\x94\x14_\x14a\x0E\xB5W_R`\x06` R_`@\x81 Ua\rCV[\x90_R`\x07` R`@_ U`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x01RV[\x03\x90\xA1c\x1A\x80\x8F\x91`\xE0\x1B\x90V[`@Q` \x81\x01\x90\x85\x82R\x83`@\x82\x01R`@\x81Ra\x0E\xD5``\x82a\r\rV[Q\x90 _\x81\x81R`\x06` R`@\x80\x82 \x80T\x94\x83R\x90\x82 \x93\x90\x93U\x90\x81R\x90Ua\rCV[\x85cjz|\x0B`\xE0\x1B_R`\x04R`$_\xFD[c\x02\xD9\xD9\xC9`\xE3\x1B_R3`\x04R`$R`D_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01iW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01iW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x01iWV[`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01iW\x90V[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01iW\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\r/W`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` R`@\x90 Ta\x0C\xD2W`\x05Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\r/W\x81a\x10\x05\x7FG\xD1\xC2*%\xBB:]NH\x1B\x9B\x1EiD\xC2\xEA\xDE1\x81\xA0\xA2\x0BI^\xD6\x1D5\xB52?$\x93a\x0C \x84`\x01` \x96\x01`\x05Ua\x0B\x7FV[`\x05T\x90`\x01\x80`\xA0\x1B\x03\x16\x90\x81_R`\x04\x83R`@_ U`@Q\x90\x81R\xA1V[_T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x075WV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\r/W`@Q\x91a\x10d`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\r\rV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01iW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x80_R`\x07` R`@_ T\x90\x81\x15a\x11)Wa\x10\x9D\x82a\x0F\x85V[\x91a\x10\xAB`@Q\x93\x84a\r\rV[\x80\x83R`\x1F\x19a\x10\xBA\x82a\x0F\x85V[\x016` \x85\x017\x80[a\x10\xCCWPP\x90V[`@Q` \x81\x01\x90\x83\x82R\x82`@\x82\x01R`@\x81Ra\x10\xEC``\x82a\r\rV[Q\x90 _R`\x06` R`@_ T\x90_\x19\x81\x01\x91\x81\x83\x11a\x0C\xBEW\x84Q\x83\x10\x15a\x0B\x97W` a\x11#\x93`\x05\x1B\x86\x01\x01Ra\rCV[\x80a\x10\xC3V[cjz|\x0B`\xE0\x1B_R`\x04R`$_\xFD[\x81\x83R\x90\x91`\x01`\x01`\xFB\x1B\x03\x83\x11a\x01iW` \x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x91\x90\x81\x10\x15a\x0B\x97W`\x05\x1B\x01\x90V[\x91\x93\x92\x93`@Qc\x12\xD4\x88\x85`\xE0\x1B\x81R` `\x04\x82\x01R` \x81\x80a\x11\x99`$\x82\x01\x87\x89a\x11;V[\x03\x81_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x12\xF6Wa\x12\xBFW[P_[\x85\x81\x10a\x12\x1FWPP\x7FO^f\xE3\xA2\xD3\xCC\xA9\xC3\xF0{M\xCE\x93/\x005\xF5'\xA8\x91w\xC5Rg\xFC\xE8\xA3\x9Ak\xB3:\x92\x93Pa\x12\x1A`@Q\x92\x83\x92` \x84R` \x84\x01\x91a\x11;V[\x03\x90\xA1V[\x80a\x12-`\x01\x92\x88\x85a\x11_V[5a\x129W[\x01a\x11\xD6V[a\x12D\x81\x88\x85a\x11_V[5_R`\x07` R`@_ T\x80a\x12]W[Pa\x123V[a\x12\xA1\x90a\x12l\x83\x8A\x87a\x11_V[5`@Q` \x81\x01\x91\x82R\x82`@\x82\x01R`@\x81Ra\x12\x8C``\x82a\r\rV[Q\x90 _R`\x06` R_`@\x81 Ua\rCV[a\x12\xAC\x82\x89\x86a\x11_V[5_R`\x07` R`@_ U_a\x12WV[` \x81=` \x11a\x12\xEEW[\x81a\x12\xD8` \x93\x83a\r\rV[\x81\x01\x03\x12a\x01iWQ\x80\x15\x15\x81\x14a\x11\xD3W_\x80\xFD[=\x91Pa\x12\xCBV[`@Q=_\x82>=\x90\xFD[\x90B\x81\x10a\x13\x84W\x81_R`\x07` R`@_ \x91\x82T\x91_\x19\x83\x14a\x0C\xBEW\x7F_\xF0>\xCC\xA1V\xE5\x0C\xD4\n\xF1f\r\xAA\xC3\x9E[\xA1\xC90\x95\x96q\xFB\xB0\xD3\xF5\xD6`\xFBx\x15\x93`\x01`@\x94\x01\x80\x91U\x83Q` \x81\x01\x91\x84\x83R\x85\x82\x01R\x84\x81Ra\x13h``\x82a\r\rV[Q\x90 _R`\x06` R\x80\x83_ U\x82Q\x91\x82R` \x82\x01R\xA1V[c\xAA/\xD9%`\xE0\x1B_R`\x04RB`$R`D_\xFD[`\xFF\x81\x14a\x13\xE0W`\xFF\x81\x16\x90`\x1F\x82\x11a\x13\xD1W`@Q\x91a\x13\xBE`@\x84a\r\rV[` \x80\x84R\x83\x81\x01\x91\x906\x837\x83RR\x90V[c,\xD4J\xC3`\xE2\x1B_R`\x04_\xFD[P`@Q_`\x02T\x80`\x01\x1C\x91`\x01\x82\x16\x91\x82\x15a\x14\xB9W[` \x84\x10\x83\x14a\x14\xA5W\x83\x85R\x84\x92\x90\x81\x15a\x14\x86WP`\x01\x14a\x14'W[a\x14$\x92P\x03\x82a\r\rV[\x90V[P`\x02_\x90\x81R\x90\x91\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE[\x81\x83\x10a\x14jWPP\x90` a\x14$\x92\x82\x01\x01a\x14\x18V[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x14RV[` \x92Pa\x14$\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01a\x14\x18V[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x92`\x7F\x16\x92a\x13\xF9V[`\xFF\x81\x14a\x14\xE7W`\xFF\x81\x16\x90`\x1F\x82\x11a\x13\xD1W`@Q\x91a\x13\xBE`@\x84a\r\rV[P`@Q_`\x03T\x80`\x01\x1C\x91`\x01\x82\x16\x91\x82\x15a\x15\x89W[` \x84\x10\x83\x14a\x14\xA5W\x83\x85R\x84\x92\x90\x81\x15a\x14\x86WP`\x01\x14a\x15*Wa\x14$\x92P\x03\x82a\r\rV[P`\x03_\x90\x81R\x90\x91\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[[\x81\x83\x10a\x15mWPP\x90` a\x14$\x92\x82\x01\x01a\x14\x18V[` \x91\x93P\x80`\x01\x91T\x83\x85\x88\x01\x01R\x01\x91\x01\x90\x91\x83\x92a\x15UV[\x92`\x7F\x16\x92a\x15\0V[\x81Q\x91\x90`A\x83\x03a\x15\xC3Wa\x15\xBC\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a\x16gV[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15a\x16-W\x80a\x15\xDFWPPV[`\x01\x81\x03a\x15\xF6Wc\xF6E\xEE\xDF`\xE0\x1B_R`\x04_\xFD[`\x02\x81\x03a\x16\x11WPc\xFC\xE6\x98\xF7`\xE0\x1B_R`\x04R`$_\xFD[`\x03\x14a\x16\x1BWPV[c5\xE2\xF3\x83`\xE2\x1B_R`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`B\x90a\x16La\x16\xE9V[\x90`@Q\x91a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a\x16\xDEW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x12\xF6W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x16\xD4W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x17\xDCW[\x15a\x17DW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q` \x81\x01\x90\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81Ra\x17\xD6`\xC0\x82a\r\rV[Q\x90 \x90V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x17\x1BV\xFE\xA1dsolcC\0\x08\x1C\0\n",
    );
    /**Custom error with signature `AlreadyUsedSig(bytes32,uint256)` and selector `0x1ed47898`.
```solidity
error AlreadyUsedSig(bytes32 attest_, uint256 nonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyUsedSig {
        pub attest_: alloy::sol_types::private::FixedBytes<32>,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadyUsedSig> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyUsedSig) -> Self {
                (value.attest_, value.nonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyUsedSig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    attest_: tuple.0,
                    nonce: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyUsedSig {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyUsedSig(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [30u8, 212u8, 120u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.attest_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                )
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
```solidity
error ECDSAInvalidSignatureLength(uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
```solidity
error ECDSAInvalidSignatureS(bytes32 s);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
        }
    };
    /**Custom error with signature `Expired(uint256,uint256)` and selector `0xaa2fd925`.
```solidity
error Expired(uint256 expiration_, uint256 currentTimestamp_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Expired {
        pub expiration_: alloy::sol_types::private::primitives::aliases::U256,
        pub currentTimestamp_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Expired> for UnderlyingRustTuple<'_> {
            fn from(value: Expired) -> Self {
                (value.expiration_, value.currentTimestamp_)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Expired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    expiration_: tuple.0,
                    currentTimestamp_: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Expired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Expired(uint256,uint256)";
            const SELECTOR: [u8; 4] = [170u8, 47u8, 217u8, 37u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiration_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentTimestamp_),
                )
            }
        }
    };
    /**Custom error with signature `ExpiredAttests(bytes32)` and selector `0x1003a2a8`.
```solidity
error ExpiredAttests(bytes32 attest_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpiredAttests {
        pub attest_: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpiredAttests> for UnderlyingRustTuple<'_> {
            fn from(value: ExpiredAttests) -> Self {
                (value.attest_,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpiredAttests {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { attest_: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpiredAttests {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpiredAttests(bytes32)";
            const SELECTOR: [u8; 4] = [16u8, 3u8, 162u8, 168u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.attest_),
                )
            }
        }
    };
    /**Custom error with signature `InvalidCaller(address,address)` and selector `0x16cece48`.
```solidity
error InvalidCaller(address caller_, address expected_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCaller {
        pub caller_: alloy::sol_types::private::Address,
        pub expected_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidCaller> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCaller) -> Self {
                (value.caller_, value.expected_)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    caller_: tuple.0,
                    expected_: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCaller(address,address)";
            const SELECTOR: [u8; 4] = [22u8, 206u8, 206u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.caller_,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.expected_,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `InvalidInput()` and selector `0xb4fa3fb3`.
```solidity
error InvalidInput();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInput {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidInput> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInput) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInput {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInput()";
            const SELECTOR: [u8; 4] = [180u8, 250u8, 63u8, 179u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidShortString()` and selector `0xb3512b0c`.
```solidity
error InvalidShortString();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidShortString {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidShortString> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidShortString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidShortString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidShortString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidShortString()";
            const SELECTOR: [u8; 4] = [179u8, 81u8, 43u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidSignature(bytes,address)` and selector `0x16001116`.
```solidity
error InvalidSignature(bytes signature_, address signer_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {
        pub signature_: alloy::sol_types::private::Bytes,
        pub signer_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                (value.signature_, value.signer_)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature_: tuple.0,
                    signer_: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature(bytes,address)";
            const SELECTOR: [u8; 4] = [22u8, 0u8, 17u8, 22u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature_,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer_,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `InvalidSigner(address)` and selector `0xbf18af43`.
```solidity
error InvalidSigner(address signer_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSigner {
        pub signer_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSigner> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSigner) -> Self {
                (value.signer_,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signer_: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSigner(address)";
            const SELECTOR: [u8; 4] = [191u8, 24u8, 175u8, 67u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer_,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
```solidity
error OwnableInvalidOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
```solidity
error OwnableUnauthorizedAccount(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OwnableUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `StringTooLong(string)` and selector `0x305a27a9`.
```solidity
error StringTooLong(string str);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StringTooLong {
        pub str: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StringTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: StringTooLong) -> Self {
                (value.str,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StringTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { str: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StringTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StringTooLong(string)";
            const SELECTOR: [u8; 4] = [48u8, 90u8, 39u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.str,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `UnregisteredAttest(bytes32)` and selector `0x6a7a7c0b`.
```solidity
error UnregisteredAttest(bytes32 attest_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnregisteredAttest {
        pub attest_: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnregisteredAttest> for UnderlyingRustTuple<'_> {
            fn from(value: UnregisteredAttest) -> Self {
                (value.attest_,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnregisteredAttest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { attest_: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnregisteredAttest {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnregisteredAttest(bytes32)";
            const SELECTOR: [u8; 4] = [106u8, 122u8, 124u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.attest_),
                )
            }
        }
    };
    /**Event with signature `AttestRegistered(bytes32,uint256)` and selector `0x5ff03ecca156e50cd40af1660daac39e5ba1c930959671fbb0d3f5d660fb7815`.
```solidity
event AttestRegistered(bytes32 attest_, uint256 expiration_);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AttestRegistered {
        #[allow(missing_docs)]
        pub attest_: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub expiration_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for AttestRegistered {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AttestRegistered(bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                95u8,
                240u8,
                62u8,
                204u8,
                161u8,
                86u8,
                229u8,
                12u8,
                212u8,
                10u8,
                241u8,
                102u8,
                13u8,
                170u8,
                195u8,
                158u8,
                91u8,
                161u8,
                201u8,
                48u8,
                149u8,
                150u8,
                113u8,
                251u8,
                176u8,
                211u8,
                245u8,
                214u8,
                96u8,
                251u8,
                120u8,
                21u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    attest_: data.0,
                    expiration_: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.attest_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiration_),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AttestRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AttestRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AttestRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Attested(address,uint256,uint256)` and selector `0xeeb125dce1d8bff72304500b7a5fb59d2cc1fdd94698d12454917b26d6a9ae90`.
```solidity
event Attested(address from_, uint256 id_, uint256 amount_);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Attested {
        #[allow(missing_docs)]
        pub from_: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub id_: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount_: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Attested {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Attested(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                238u8,
                177u8,
                37u8,
                220u8,
                225u8,
                216u8,
                191u8,
                247u8,
                35u8,
                4u8,
                80u8,
                11u8,
                122u8,
                95u8,
                181u8,
                157u8,
                44u8,
                193u8,
                253u8,
                217u8,
                70u8,
                152u8,
                209u8,
                36u8,
                84u8,
                145u8,
                123u8,
                38u8,
                214u8,
                169u8,
                174u8,
                144u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    from_: data.0,
                    id_: data.1,
                    amount_: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from_,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount_),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Attested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Attested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Attested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EIP712DomainChanged()` and selector `0x0a6387c9ea3628b88a633bb4f3b151770f70085117a15f9bf3787cda53f13d31`.
```solidity
event EIP712DomainChanged();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EIP712DomainChanged {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EIP712DomainChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EIP712DomainChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EIP712DomainChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EIP712DomainChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NoncesConsumed(uint256[])` and selector `0x4f5e66e3a2d3cca9c3f07b4dce932f0035f527a89177c55267fce8a39a6bb33a`.
```solidity
event NoncesConsumed(uint256[] nonces_);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NoncesConsumed {
        #[allow(missing_docs)]
        pub nonces_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NoncesConsumed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NoncesConsumed(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                79u8,
                94u8,
                102u8,
                227u8,
                162u8,
                211u8,
                204u8,
                169u8,
                195u8,
                240u8,
                123u8,
                77u8,
                206u8,
                147u8,
                47u8,
                0u8,
                53u8,
                245u8,
                39u8,
                168u8,
                145u8,
                119u8,
                197u8,
                82u8,
                103u8,
                252u8,
                232u8,
                163u8,
                154u8,
                107u8,
                179u8,
                58u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { nonces_: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonces_),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NoncesConsumed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NoncesConsumed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NoncesConsumed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferStarted(address,address)` and selector `0x38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e22700`.
```solidity
event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferStarted {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferStarted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferStarted(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                56u8,
                209u8,
                107u8,
                140u8,
                172u8,
                34u8,
                217u8,
                159u8,
                199u8,
                193u8,
                36u8,
                185u8,
                205u8,
                13u8,
                226u8,
                211u8,
                250u8,
                31u8,
                174u8,
                244u8,
                32u8,
                191u8,
                231u8,
                145u8,
                216u8,
                195u8,
                98u8,
                215u8,
                101u8,
                226u8,
                39u8,
                0u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OwnershipTransferStarted,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SignerAdded(address)` and selector `0x47d1c22a25bb3a5d4e481b9b1e6944c2eade3181a0a20b495ed61d35b5323f24`.
```solidity
event SignerAdded(address signer_);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SignerAdded {
        #[allow(missing_docs)]
        pub signer_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SignerAdded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SignerAdded(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                71u8,
                209u8,
                194u8,
                42u8,
                37u8,
                187u8,
                58u8,
                93u8,
                78u8,
                72u8,
                27u8,
                155u8,
                30u8,
                105u8,
                68u8,
                194u8,
                234u8,
                222u8,
                49u8,
                129u8,
                160u8,
                162u8,
                11u8,
                73u8,
                94u8,
                214u8,
                29u8,
                53u8,
                181u8,
                50u8,
                63u8,
                36u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { signer_: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer_,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SignerAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SignerAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SignerAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SignerRemoved(address)` and selector `0x3525e22824a8a7df2c9a6029941c824cf95b6447f1e13d5128fd3826d35afe8b`.
```solidity
event SignerRemoved(address signer_);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SignerRemoved {
        #[allow(missing_docs)]
        pub signer_: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SignerRemoved {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SignerRemoved(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                53u8,
                37u8,
                226u8,
                40u8,
                36u8,
                168u8,
                167u8,
                223u8,
                44u8,
                154u8,
                96u8,
                41u8,
                148u8,
                28u8,
                130u8,
                76u8,
                249u8,
                91u8,
                100u8,
                71u8,
                241u8,
                225u8,
                61u8,
                81u8,
                40u8,
                253u8,
                56u8,
                38u8,
                211u8,
                90u8,
                254u8,
                139u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { signer_: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer_,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SignerRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SignerRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SignerRemoved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address owner_, address compactContract_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub owner_: alloy::sol_types::private::Address,
        pub compactContract_: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.owner_, value.compactContract_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner_: tuple.0,
                        compactContract_: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner_,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.compactContract_,
                    ),
                )
            }
        }
    };
    /**Function with signature `acceptOwnership()` and selector `0x79ba5097`.
```solidity
function acceptOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipCall {}
    ///Container type for the return parameters of the [`acceptOwnership()`](acceptOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<acceptOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: acceptOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acceptOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<acceptOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptOwnership()";
            const SELECTOR: [u8; 4] = [121u8, 186u8, 80u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `addSigner(address)` and selector `0xeb12d61e`.
```solidity
function addSigner(address signer_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSignerCall {
        pub signer_: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addSigner(address)`](addSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSignerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addSignerCall> for UnderlyingRustTuple<'_> {
                fn from(value: addSignerCall) -> Self {
                    (value.signer_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signer_: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addSignerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addSignerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSignerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSigner(address)";
            const SELECTOR: [u8; 4] = [235u8, 18u8, 214u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer_,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `attest(address,address,address,uint256,uint256)` and selector `0x1a808f91`.
```solidity
function attest(address, address from_, address, uint256 id_, uint256 amount_) external returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct attestCall {
        pub _0: alloy::sol_types::private::Address,
        pub from_: alloy::sol_types::private::Address,
        pub _2: alloy::sol_types::private::Address,
        pub id_: alloy::sol_types::private::primitives::aliases::U256,
        pub amount_: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`attest(address,address,address,uint256,uint256)`](attestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct attestReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<attestCall> for UnderlyingRustTuple<'_> {
                fn from(value: attestCall) -> Self {
                    (value._0, value.from_, value._2, value.id_, value.amount_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for attestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        from_: tuple.1,
                        _2: tuple.2,
                        id_: tuple.3,
                        amount_: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<attestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: attestReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for attestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for attestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = attestReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "attest(address,address,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [26u8, 128u8, 143u8, 145u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from_,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._2,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount_),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `checkAttestExpirations(bytes32)` and selector `0x254204c5`.
```solidity
function checkAttestExpirations(bytes32 attest_) external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAttestExpirations_0Call {
        pub attest_: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`checkAttestExpirations(bytes32)`](checkAttestExpirations_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAttestExpirations_0Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkAttestExpirations_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkAttestExpirations_0Call) -> Self {
                    (value.attest_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAttestExpirations_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { attest_: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkAttestExpirations_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkAttestExpirations_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAttestExpirations_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkAttestExpirations_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkAttestExpirations_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkAttestExpirations(bytes32)";
            const SELECTOR: [u8; 4] = [37u8, 66u8, 4u8, 197u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.attest_),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `checkAttestExpirations(address,uint256,uint256)` and selector `0xfc79101e`.
```solidity
function checkAttestExpirations(address sponsor_, uint256 id_, uint256 amount_) external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAttestExpirations_1Call {
        pub sponsor_: alloy::sol_types::private::Address,
        pub id_: alloy::sol_types::private::primitives::aliases::U256,
        pub amount_: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`checkAttestExpirations(address,uint256,uint256)`](checkAttestExpirations_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAttestExpirations_1Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkAttestExpirations_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkAttestExpirations_1Call) -> Self {
                    (value.sponsor_, value.id_, value.amount_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAttestExpirations_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sponsor_: tuple.0,
                        id_: tuple.1,
                        amount_: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkAttestExpirations_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkAttestExpirations_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAttestExpirations_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkAttestExpirations_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkAttestExpirations_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkAttestExpirations(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [252u8, 121u8, 16u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sponsor_,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount_),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `checkIfSigner(address)` and selector `0xf780c0d5`.
```solidity
function checkIfSigner(address signer_) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkIfSignerCall {
        pub signer_: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`checkIfSigner(address)`](checkIfSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkIfSignerReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkIfSignerCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkIfSignerCall) -> Self {
                    (value.signer_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkIfSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signer_: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkIfSignerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkIfSignerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkIfSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkIfSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkIfSignerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkIfSigner(address)";
            const SELECTOR: [u8; 4] = [247u8, 128u8, 192u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer_,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `consume(uint256[],bytes32[])` and selector `0xc9d0fa86`.
```solidity
function consume(uint256[] memory nonces_, bytes32[] memory attests_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct consumeCall {
        pub nonces_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub attests_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`consume(uint256[],bytes32[])`](consumeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct consumeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<consumeCall> for UnderlyingRustTuple<'_> {
                fn from(value: consumeCall) -> Self {
                    (value.nonces_, value.attests_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for consumeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        nonces_: tuple.0,
                        attests_: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<consumeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: consumeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for consumeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for consumeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = consumeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "consume(uint256[],bytes32[])";
            const SELECTOR: [u8; 4] = [201u8, 208u8, 250u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonces_),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.attests_),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `consumeViaSignature((address,uint256[],bytes32[]),bytes)` and selector `0x2bca447f`.
```solidity
function consumeViaSignature(IServerAllocator.NonceConsumption memory data_, bytes memory signature_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct consumeViaSignatureCall {
        pub data_: <IServerAllocator::NonceConsumption as alloy::sol_types::SolType>::RustType,
        pub signature_: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`consumeViaSignature((address,uint256[],bytes32[]),bytes)`](consumeViaSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct consumeViaSignatureReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IServerAllocator::NonceConsumption,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IServerAllocator::NonceConsumption as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<consumeViaSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: consumeViaSignatureCall) -> Self {
                    (value.data_, value.signature_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for consumeViaSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        data_: tuple.0,
                        signature_: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<consumeViaSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: consumeViaSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for consumeViaSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for consumeViaSignatureCall {
            type Parameters<'a> = (
                IServerAllocator::NonceConsumption,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = consumeViaSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "consumeViaSignature((address,uint256[],bytes32[]),bytes)";
            const SELECTOR: [u8; 4] = [43u8, 202u8, 68u8, 127u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IServerAllocator::NonceConsumption as alloy_sol_types::SolType>::tokenize(
                        &self.data_,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature_,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        pub name: alloy::sol_types::private::String,
        pub version: alloy::sol_types::private::String,
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        pub verifyingContract: alloy::sol_types::private::Address,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub extensions: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getAllSigners()` and selector `0xd42f2f35`.
```solidity
function getAllSigners() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllSignersCall {}
    ///Container type for the return parameters of the [`getAllSigners()`](getAllSignersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllSignersReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllSignersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllSignersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllSignersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllSignersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllSignersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllSignersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllSignersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllSignersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllSigners()";
            const SELECTOR: [u8; 4] = [212u8, 47u8, 47u8, 53u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getCompactContract()` and selector `0xd6996b6e`.
```solidity
function getCompactContract() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompactContractCall {}
    ///Container type for the return parameters of the [`getCompactContract()`](getCompactContractCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompactContractReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCompactContractCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompactContractCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompactContractCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCompactContractReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompactContractReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompactContractReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCompactContractCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCompactContractReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCompactContract()";
            const SELECTOR: [u8; 4] = [214u8, 153u8, 107u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`.
```solidity
function isValidSignature(bytes32 hash_, bytes memory signature_) external view returns (bytes4 magicValue);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        pub hash_: alloy::sol_types::private::FixedBytes<32>,
        pub signature_: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isValidSignature(bytes32,bytes)`](isValidSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureReturn {
        pub magicValue: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value.hash_, value.signature_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        hash_: tuple.0,
                        signature_: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value.magicValue,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { magicValue: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidSignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidSignature(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [22u8, 38u8, 186u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hash_),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature_,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `pendingOwner()` and selector `0xe30c3978`.
```solidity
function pendingOwner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingOwnerCall {}
    ///Container type for the return parameters of the [`pendingOwner()`](pendingOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingOwnerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pendingOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: pendingOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pendingOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pendingOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pendingOwnerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingOwner()";
            const SELECTOR: [u8; 4] = [227u8, 12u8, 57u8, 120u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `registerAttest(bytes32,uint256)` and selector `0x2df97546`.
```solidity
function registerAttest(bytes32 attest_, uint256 expiration_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAttestCall {
        pub attest_: alloy::sol_types::private::FixedBytes<32>,
        pub expiration_: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`registerAttest(bytes32,uint256)`](registerAttestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAttestReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAttestCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerAttestCall) -> Self {
                    (value.attest_, value.expiration_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerAttestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        attest_: tuple.0,
                        expiration_: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAttestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAttestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAttestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerAttestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerAttestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerAttest(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [45u8, 249u8, 117u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.attest_),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiration_),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `registerAttestViaSignature((address,bytes32,uint256,uint256),bytes)` and selector `0xcfde4374`.
```solidity
function registerAttestViaSignature(IServerAllocator.RegisterAttest memory attest_, bytes memory signature_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAttestViaSignatureCall {
        pub attest_: <IServerAllocator::RegisterAttest as alloy::sol_types::SolType>::RustType,
        pub signature_: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerAttestViaSignature((address,bytes32,uint256,uint256),bytes)`](registerAttestViaSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAttestViaSignatureReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IServerAllocator::RegisterAttest,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IServerAllocator::RegisterAttest as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAttestViaSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAttestViaSignatureCall) -> Self {
                    (value.attest_, value.signature_)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAttestViaSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        attest_: tuple.0,
                        signature_: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAttestViaSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAttestViaSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAttestViaSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerAttestViaSignatureCall {
            type Parameters<'a> = (
                IServerAllocator::RegisterAttest,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerAttestViaSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerAttestViaSignature((address,bytes32,uint256,uint256),bytes)";
            const SELECTOR: [u8; 4] = [207u8, 222u8, 67u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IServerAllocator::RegisterAttest as alloy_sol_types::SolType>::tokenize(
                        &self.attest_,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature_,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `removeSigner(address)` and selector `0x0e316ab7`.
```solidity
function removeSigner(address signer_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeSignerCall {
        pub signer_: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeSigner(address)`](removeSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeSignerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeSignerCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeSignerCall) -> Self {
                    (value.signer_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signer_: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeSignerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeSignerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeSignerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeSigner(address)";
            const SELECTOR: [u8; 4] = [14u8, 49u8, 106u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer_,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`ServerAllocator`](self) function calls.
    pub enum ServerAllocatorCalls {
        acceptOwnership(acceptOwnershipCall),
        addSigner(addSignerCall),
        attest(attestCall),
        checkAttestExpirations_0(checkAttestExpirations_0Call),
        checkAttestExpirations_1(checkAttestExpirations_1Call),
        checkIfSigner(checkIfSignerCall),
        consume(consumeCall),
        consumeViaSignature(consumeViaSignatureCall),
        eip712Domain(eip712DomainCall),
        getAllSigners(getAllSignersCall),
        getCompactContract(getCompactContractCall),
        isValidSignature(isValidSignatureCall),
        owner(ownerCall),
        pendingOwner(pendingOwnerCall),
        registerAttest(registerAttestCall),
        registerAttestViaSignature(registerAttestViaSignatureCall),
        removeSigner(removeSignerCall),
        renounceOwnership(renounceOwnershipCall),
        transferOwnership(transferOwnershipCall),
    }
    #[automatically_derived]
    impl ServerAllocatorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [14u8, 49u8, 106u8, 183u8],
            [22u8, 38u8, 186u8, 126u8],
            [26u8, 128u8, 143u8, 145u8],
            [37u8, 66u8, 4u8, 197u8],
            [43u8, 202u8, 68u8, 127u8],
            [45u8, 249u8, 117u8, 70u8],
            [113u8, 80u8, 24u8, 166u8],
            [121u8, 186u8, 80u8, 151u8],
            [132u8, 176u8, 25u8, 110u8],
            [141u8, 165u8, 203u8, 91u8],
            [201u8, 208u8, 250u8, 134u8],
            [207u8, 222u8, 67u8, 116u8],
            [212u8, 47u8, 47u8, 53u8],
            [214u8, 153u8, 107u8, 110u8],
            [227u8, 12u8, 57u8, 120u8],
            [235u8, 18u8, 214u8, 30u8],
            [242u8, 253u8, 227u8, 139u8],
            [247u8, 128u8, 192u8, 213u8],
            [252u8, 121u8, 16u8, 30u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ServerAllocatorCalls {
        const NAME: &'static str = "ServerAllocatorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 19usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::acceptOwnership(_) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSigner(_) => {
                    <addSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::attest(_) => <attestCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::checkAttestExpirations_0(_) => {
                    <checkAttestExpirations_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkAttestExpirations_1(_) => {
                    <checkAttestExpirations_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkIfSigner(_) => {
                    <checkIfSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::consume(_) => <consumeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::consumeViaSignature(_) => {
                    <consumeViaSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllSigners(_) => {
                    <getAllSignersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCompactContract(_) => {
                    <getCompactContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pendingOwner(_) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAttest(_) => {
                    <registerAttestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAttestViaSignature(_) => {
                    <registerAttestViaSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeSigner(_) => {
                    <removeSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<ServerAllocatorCalls>] = &[
                {
                    fn removeSigner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <removeSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::removeSigner)
                    }
                    removeSigner
                },
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::isValidSignature)
                    }
                    isValidSignature
                },
                {
                    fn attest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <attestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::attest)
                    }
                    attest
                },
                {
                    fn checkAttestExpirations_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <checkAttestExpirations_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::checkAttestExpirations_0)
                    }
                    checkAttestExpirations_0
                },
                {
                    fn consumeViaSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <consumeViaSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::consumeViaSignature)
                    }
                    consumeViaSignature
                },
                {
                    fn registerAttest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <registerAttestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::registerAttest)
                    }
                    registerAttest
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn acceptOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::acceptOwnership)
                    }
                    acceptOwnership
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::owner)
                    }
                    owner
                },
                {
                    fn consume(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <consumeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::consume)
                    }
                    consume
                },
                {
                    fn registerAttestViaSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <registerAttestViaSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::registerAttestViaSignature)
                    }
                    registerAttestViaSignature
                },
                {
                    fn getAllSigners(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <getAllSignersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::getAllSigners)
                    }
                    getAllSigners
                },
                {
                    fn getCompactContract(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <getCompactContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::getCompactContract)
                    }
                    getCompactContract
                },
                {
                    fn pendingOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <pendingOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::pendingOwner)
                    }
                    pendingOwner
                },
                {
                    fn addSigner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <addSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::addSigner)
                    }
                    addSigner
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn checkIfSigner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <checkIfSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::checkIfSigner)
                    }
                    checkIfSigner
                },
                {
                    fn checkAttestExpirations_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorCalls> {
                        <checkAttestExpirations_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorCalls::checkAttestExpirations_1)
                    }
                    checkAttestExpirations_1
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::acceptOwnership(inner) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSigner(inner) => {
                    <addSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::attest(inner) => {
                    <attestCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkAttestExpirations_0(inner) => {
                    <checkAttestExpirations_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkAttestExpirations_1(inner) => {
                    <checkAttestExpirations_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkIfSigner(inner) => {
                    <checkIfSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::consume(inner) => {
                    <consumeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::consumeViaSignature(inner) => {
                    <consumeViaSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllSigners(inner) => {
                    <getAllSignersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCompactContract(inner) => {
                    <getCompactContractCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pendingOwner(inner) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerAttest(inner) => {
                    <registerAttestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerAttestViaSignature(inner) => {
                    <registerAttestViaSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeSigner(inner) => {
                    <removeSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::acceptOwnership(inner) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSigner(inner) => {
                    <addSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::attest(inner) => {
                    <attestCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkAttestExpirations_0(inner) => {
                    <checkAttestExpirations_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkAttestExpirations_1(inner) => {
                    <checkAttestExpirations_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkIfSigner(inner) => {
                    <checkIfSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::consume(inner) => {
                    <consumeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::consumeViaSignature(inner) => {
                    <consumeViaSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllSigners(inner) => {
                    <getAllSignersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCompactContract(inner) => {
                    <getCompactContractCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pendingOwner(inner) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerAttest(inner) => {
                    <registerAttestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerAttestViaSignature(inner) => {
                    <registerAttestViaSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeSigner(inner) => {
                    <removeSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ServerAllocator`](self) custom errors.
    pub enum ServerAllocatorErrors {
        AlreadyUsedSig(AlreadyUsedSig),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        Expired(Expired),
        ExpiredAttests(ExpiredAttests),
        InvalidCaller(InvalidCaller),
        InvalidInput(InvalidInput),
        InvalidShortString(InvalidShortString),
        InvalidSignature(InvalidSignature),
        InvalidSigner(InvalidSigner),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        StringTooLong(StringTooLong),
        UnregisteredAttest(UnregisteredAttest),
    }
    #[automatically_derived]
    impl ServerAllocatorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [16u8, 3u8, 162u8, 168u8],
            [17u8, 140u8, 218u8, 167u8],
            [22u8, 0u8, 17u8, 22u8],
            [22u8, 206u8, 206u8, 72u8],
            [30u8, 79u8, 189u8, 247u8],
            [30u8, 212u8, 120u8, 152u8],
            [48u8, 90u8, 39u8, 169u8],
            [106u8, 122u8, 124u8, 11u8],
            [170u8, 47u8, 217u8, 37u8],
            [179u8, 81u8, 43u8, 12u8],
            [180u8, 250u8, 63u8, 179u8],
            [191u8, 24u8, 175u8, 67u8],
            [215u8, 139u8, 206u8, 12u8],
            [246u8, 69u8, 238u8, 223u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ServerAllocatorErrors {
        const NAME: &'static str = "ServerAllocatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyUsedSig(_) => {
                    <AlreadyUsedSig as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Expired(_) => <Expired as alloy_sol_types::SolError>::SELECTOR,
                Self::ExpiredAttests(_) => {
                    <ExpiredAttests as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCaller(_) => {
                    <InvalidCaller as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInput(_) => {
                    <InvalidInput as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSigner(_) => {
                    <InvalidSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => {
                    <StringTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnregisteredAttest(_) => {
                    <UnregisteredAttest as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<ServerAllocatorErrors>] = &[
                {
                    fn ExpiredAttests(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <ExpiredAttests as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::ExpiredAttests)
                    }
                    ExpiredAttests
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <InvalidCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::InvalidCaller)
                    }
                    InvalidCaller
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn AlreadyUsedSig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <AlreadyUsedSig as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::AlreadyUsedSig)
                    }
                    AlreadyUsedSig
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn UnregisteredAttest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <UnregisteredAttest as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::UnregisteredAttest)
                    }
                    UnregisteredAttest
                },
                {
                    fn Expired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <Expired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::Expired)
                    }
                    Expired
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn InvalidInput(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <InvalidInput as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::InvalidInput)
                    }
                    InvalidInput
                },
                {
                    fn InvalidSigner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::InvalidSigner)
                    }
                    InvalidSigner
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ServerAllocatorErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ServerAllocatorErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AlreadyUsedSig(inner) => {
                    <AlreadyUsedSig as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpiredAttests(inner) => {
                    <ExpiredAttests as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidInput(inner) => {
                    <InvalidInput as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSigner(inner) => {
                    <InvalidSigner as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::UnregisteredAttest(inner) => {
                    <UnregisteredAttest as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyUsedSig(inner) => {
                    <AlreadyUsedSig as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ExpiredAttests(inner) => {
                    <ExpiredAttests as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidInput(inner) => {
                    <InvalidInput as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSigner(inner) => {
                    <InvalidSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnregisteredAttest(inner) => {
                    <UnregisteredAttest as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ServerAllocator`](self) events.
    pub enum ServerAllocatorEvents {
        AttestRegistered(AttestRegistered),
        Attested(Attested),
        EIP712DomainChanged(EIP712DomainChanged),
        NoncesConsumed(NoncesConsumed),
        OwnershipTransferStarted(OwnershipTransferStarted),
        OwnershipTransferred(OwnershipTransferred),
        SignerAdded(SignerAdded),
        SignerRemoved(SignerRemoved),
    }
    #[automatically_derived]
    impl ServerAllocatorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8,
                99u8,
                135u8,
                201u8,
                234u8,
                54u8,
                40u8,
                184u8,
                138u8,
                99u8,
                59u8,
                180u8,
                243u8,
                177u8,
                81u8,
                119u8,
                15u8,
                112u8,
                8u8,
                81u8,
                23u8,
                161u8,
                95u8,
                155u8,
                243u8,
                120u8,
                124u8,
                218u8,
                83u8,
                241u8,
                61u8,
                49u8,
            ],
            [
                53u8,
                37u8,
                226u8,
                40u8,
                36u8,
                168u8,
                167u8,
                223u8,
                44u8,
                154u8,
                96u8,
                41u8,
                148u8,
                28u8,
                130u8,
                76u8,
                249u8,
                91u8,
                100u8,
                71u8,
                241u8,
                225u8,
                61u8,
                81u8,
                40u8,
                253u8,
                56u8,
                38u8,
                211u8,
                90u8,
                254u8,
                139u8,
            ],
            [
                56u8,
                209u8,
                107u8,
                140u8,
                172u8,
                34u8,
                217u8,
                159u8,
                199u8,
                193u8,
                36u8,
                185u8,
                205u8,
                13u8,
                226u8,
                211u8,
                250u8,
                31u8,
                174u8,
                244u8,
                32u8,
                191u8,
                231u8,
                145u8,
                216u8,
                195u8,
                98u8,
                215u8,
                101u8,
                226u8,
                39u8,
                0u8,
            ],
            [
                71u8,
                209u8,
                194u8,
                42u8,
                37u8,
                187u8,
                58u8,
                93u8,
                78u8,
                72u8,
                27u8,
                155u8,
                30u8,
                105u8,
                68u8,
                194u8,
                234u8,
                222u8,
                49u8,
                129u8,
                160u8,
                162u8,
                11u8,
                73u8,
                94u8,
                214u8,
                29u8,
                53u8,
                181u8,
                50u8,
                63u8,
                36u8,
            ],
            [
                79u8,
                94u8,
                102u8,
                227u8,
                162u8,
                211u8,
                204u8,
                169u8,
                195u8,
                240u8,
                123u8,
                77u8,
                206u8,
                147u8,
                47u8,
                0u8,
                53u8,
                245u8,
                39u8,
                168u8,
                145u8,
                119u8,
                197u8,
                82u8,
                103u8,
                252u8,
                232u8,
                163u8,
                154u8,
                107u8,
                179u8,
                58u8,
            ],
            [
                95u8,
                240u8,
                62u8,
                204u8,
                161u8,
                86u8,
                229u8,
                12u8,
                212u8,
                10u8,
                241u8,
                102u8,
                13u8,
                170u8,
                195u8,
                158u8,
                91u8,
                161u8,
                201u8,
                48u8,
                149u8,
                150u8,
                113u8,
                251u8,
                176u8,
                211u8,
                245u8,
                214u8,
                96u8,
                251u8,
                120u8,
                21u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                238u8,
                177u8,
                37u8,
                220u8,
                225u8,
                216u8,
                191u8,
                247u8,
                35u8,
                4u8,
                80u8,
                11u8,
                122u8,
                95u8,
                181u8,
                157u8,
                44u8,
                193u8,
                253u8,
                217u8,
                70u8,
                152u8,
                209u8,
                36u8,
                84u8,
                145u8,
                123u8,
                38u8,
                214u8,
                169u8,
                174u8,
                144u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ServerAllocatorEvents {
        const NAME: &'static str = "ServerAllocatorEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AttestRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AttestRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AttestRegistered)
                }
                Some(<Attested as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Attested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Attested)
                }
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(<NoncesConsumed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NoncesConsumed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NoncesConsumed)
                }
                Some(
                    <OwnershipTransferStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferStarted)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<SignerAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SignerAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SignerAdded)
                }
                Some(<SignerRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SignerRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SignerRemoved)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for ServerAllocatorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AttestRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Attested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NoncesConsumed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SignerAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SignerRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AttestRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Attested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NoncesConsumed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SignerAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SignerRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ServerAllocator`](self) contract instance.

See the [wrapper's documentation](`ServerAllocatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ServerAllocatorInstance<T, P, N> {
        ServerAllocatorInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        owner_: alloy::sol_types::private::Address,
        compactContract_: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ServerAllocatorInstance<T, P, N>>,
    > {
        ServerAllocatorInstance::<T, P, N>::deploy(provider, owner_, compactContract_)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        owner_: alloy::sol_types::private::Address,
        compactContract_: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ServerAllocatorInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, owner_, compactContract_)
    }
    /**A [`ServerAllocator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ServerAllocator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ServerAllocatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ServerAllocatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ServerAllocatorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ServerAllocatorInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ServerAllocator`](self) contract instance.

See the [wrapper's documentation](`ServerAllocatorInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            owner_: alloy::sol_types::private::Address,
            compactContract_: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ServerAllocatorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, owner_, compactContract_);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            owner_: alloy::sol_types::private::Address,
            compactContract_: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            owner_,
                            compactContract_,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ServerAllocatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ServerAllocatorInstance<T, P, N> {
            ServerAllocatorInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ServerAllocatorInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`acceptOwnership`] function.
        pub fn acceptOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptOwnershipCall, N> {
            self.call_builder(&acceptOwnershipCall {})
        }
        ///Creates a new call builder for the [`addSigner`] function.
        pub fn addSigner(
            &self,
            signer_: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, addSignerCall, N> {
            self.call_builder(&addSignerCall { signer_ })
        }
        ///Creates a new call builder for the [`attest`] function.
        pub fn attest(
            &self,
            _0: alloy::sol_types::private::Address,
            from_: alloy::sol_types::private::Address,
            _2: alloy::sol_types::private::Address,
            id_: alloy::sol_types::private::primitives::aliases::U256,
            amount_: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, attestCall, N> {
            self.call_builder(
                &attestCall {
                    _0,
                    from_,
                    _2,
                    id_,
                    amount_,
                },
            )
        }
        ///Creates a new call builder for the [`checkAttestExpirations_0`] function.
        pub fn checkAttestExpirations_0(
            &self,
            attest_: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkAttestExpirations_0Call, N> {
            self.call_builder(
                &checkAttestExpirations_0Call {
                    attest_,
                },
            )
        }
        ///Creates a new call builder for the [`checkAttestExpirations_1`] function.
        pub fn checkAttestExpirations_1(
            &self,
            sponsor_: alloy::sol_types::private::Address,
            id_: alloy::sol_types::private::primitives::aliases::U256,
            amount_: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkAttestExpirations_1Call, N> {
            self.call_builder(
                &checkAttestExpirations_1Call {
                    sponsor_,
                    id_,
                    amount_,
                },
            )
        }
        ///Creates a new call builder for the [`checkIfSigner`] function.
        pub fn checkIfSigner(
            &self,
            signer_: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkIfSignerCall, N> {
            self.call_builder(&checkIfSignerCall { signer_ })
        }
        ///Creates a new call builder for the [`consume`] function.
        pub fn consume(
            &self,
            nonces_: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            attests_: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, consumeCall, N> {
            self.call_builder(&consumeCall { nonces_, attests_ })
        }
        ///Creates a new call builder for the [`consumeViaSignature`] function.
        pub fn consumeViaSignature(
            &self,
            data_: <IServerAllocator::NonceConsumption as alloy::sol_types::SolType>::RustType,
            signature_: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, consumeViaSignatureCall, N> {
            self.call_builder(
                &consumeViaSignatureCall {
                    data_,
                    signature_,
                },
            )
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`getAllSigners`] function.
        pub fn getAllSigners(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllSignersCall, N> {
            self.call_builder(&getAllSignersCall {})
        }
        ///Creates a new call builder for the [`getCompactContract`] function.
        pub fn getCompactContract(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCompactContractCall, N> {
            self.call_builder(&getCompactContractCall {})
        }
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            hash_: alloy::sol_types::private::FixedBytes<32>,
            signature_: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(
                &isValidSignatureCall {
                    hash_,
                    signature_,
                },
            )
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`pendingOwner`] function.
        pub fn pendingOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingOwnerCall, N> {
            self.call_builder(&pendingOwnerCall {})
        }
        ///Creates a new call builder for the [`registerAttest`] function.
        pub fn registerAttest(
            &self,
            attest_: alloy::sol_types::private::FixedBytes<32>,
            expiration_: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAttestCall, N> {
            self.call_builder(
                &registerAttestCall {
                    attest_,
                    expiration_,
                },
            )
        }
        ///Creates a new call builder for the [`registerAttestViaSignature`] function.
        pub fn registerAttestViaSignature(
            &self,
            attest_: <IServerAllocator::RegisterAttest as alloy::sol_types::SolType>::RustType,
            signature_: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAttestViaSignatureCall, N> {
            self.call_builder(
                &registerAttestViaSignatureCall {
                    attest_,
                    signature_,
                },
            )
        }
        ///Creates a new call builder for the [`removeSigner`] function.
        pub fn removeSigner(
            &self,
            signer_: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeSignerCall, N> {
            self.call_builder(&removeSignerCall { signer_ })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ServerAllocatorInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AttestRegistered`] event.
        pub fn AttestRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AttestRegistered, N> {
            self.event_filter::<AttestRegistered>()
        }
        ///Creates a new event filter for the [`Attested`] event.
        pub fn Attested_filter(&self) -> alloy_contract::Event<T, &P, Attested, N> {
            self.event_filter::<Attested>()
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`NoncesConsumed`] event.
        pub fn NoncesConsumed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NoncesConsumed, N> {
            self.event_filter::<NoncesConsumed>()
        }
        ///Creates a new event filter for the [`OwnershipTransferStarted`] event.
        pub fn OwnershipTransferStarted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferStarted, N> {
            self.event_filter::<OwnershipTransferStarted>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`SignerAdded`] event.
        pub fn SignerAdded_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SignerAdded, N> {
            self.event_filter::<SignerAdded>()
        }
        ///Creates a new event filter for the [`SignerRemoved`] event.
        pub fn SignerRemoved_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SignerRemoved, N> {
            self.event_filter::<SignerRemoved>()
        }
    }
}
