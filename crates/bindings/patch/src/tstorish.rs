/**

Generated by the following Solidity interface...
```solidity
interface Tstorish {
    error OnlyDirectCalls();
    error TStoreAlreadyActivated();
    error TStoreNotSupported();
    error TloadTestContractDeploymentFailed();

    constructor();

    function __activateTstore() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "__activateTstore",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "error",
    "name": "OnlyDirectCalls",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TStoreAlreadyActivated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TStoreNotSupported",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TloadTestContractDeploymentFailed",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod Tstorish {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610120604052346100fb57696002601e613d5c3d52f35f52600a60165ff06001600160a01b038116156100ec575f80808084600a5a04fa3d156100e7573d6001600160401b0381116100e25760405190601f8101601f19908116603f011682016001600160401b038111838210176100e25760405281525f60203d92013e5b80156100cd57600160c052600360e0526005610100525b60a0526080526040516101509081610114823960805181606d015260a05181603d015260c05181505060e05181505061010051815050f35b600260c052600460e052600661010052610095565b6100ff565b61007e565b632aea588760e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c637423eb3c14610024575f80fd5b3461013f575f36600319011261013f57323303610130577f00000000000000000000000000000000000000000000000000000000000000008015610125575b610116575f8080807f0000000000000000000000000000000000000000000000000000000000000000600a5a04fa3d15610111573d67ffffffffffffffff81116100fd5760405190601f8101601f19908116603f0116820167ffffffffffffffff8111838210176100fd5760405281525f60203d92013e5b156100ee575f805460ff19166001179055005b6370a4078f60e01b5f5260045ffd5b634e487b7160e01b5f52604160045260245ffd5b6100db565b630f45b98b60e41b5f5260045ffd5b5060ff5f5416610063565b63096650c560e21b5f5260045ffd5b5f80fdfea164736f6c634300081c000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01 `@R4a\0\xFBWi`\x02`\x1Ea=\\=R\xF3_R`\n`\x16_\xF0`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\0\xECW_\x80\x80\x80\x84`\nZ\x04\xFA=\x15a\0\xE7W=`\x01`\x01`@\x1B\x03\x81\x11a\0\xE2W`@Q\x90`\x1F\x81\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x82\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17a\0\xE2W`@R\x81R_` =\x92\x01>[\x80\x15a\0\xCDW`\x01`\xC0R`\x03`\xE0R`\x05a\x01\0R[`\xA0R`\x80R`@Qa\x01P\x90\x81a\x01\x14\x829`\x80Q\x81`m\x01R`\xA0Q\x81`=\x01R`\xC0Q\x81PP`\xE0Q\x81PPa\x01\0Q\x81PP\xF3[`\x02`\xC0R`\x04`\xE0R`\x06a\x01\0Ra\0\x95V[a\0\xFFV[a\0~V[c*\xEAX\x87`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1Cct#\xEB<\x14a\0$W_\x80\xFD[4a\x01?W_6`\x03\x19\x01\x12a\x01?W23\x03a\x010W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x15a\x01%W[a\x01\x16W_\x80\x80\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\nZ\x04\xFA=\x15a\x01\x11W=g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDW`@Q\x90`\x1F\x81\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\0\xFDW`@R\x81R_` =\x92\x01>[\x15a\0\xEEW_\x80T`\xFF\x19\x16`\x01\x17\x90U\0[cp\xA4\x07\x8F`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[a\0\xDBV[c\x0FE\xB9\x8B`\xE4\x1B_R`\x04_\xFD[P`\xFF_T\x16a\0cV[c\tfP\xC5`\xE2\x1B_R`\x04_\xFD[_\x80\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c637423eb3c14610024575f80fd5b3461013f575f36600319011261013f57323303610130577f00000000000000000000000000000000000000000000000000000000000000008015610125575b610116575f8080807f0000000000000000000000000000000000000000000000000000000000000000600a5a04fa3d15610111573d67ffffffffffffffff81116100fd5760405190601f8101601f19908116603f0116820167ffffffffffffffff8111838210176100fd5760405281525f60203d92013e5b156100ee575f805460ff19166001179055005b6370a4078f60e01b5f5260045ffd5b634e487b7160e01b5f52604160045260245ffd5b6100db565b630f45b98b60e41b5f5260045ffd5b5060ff5f5416610063565b63096650c560e21b5f5260045ffd5b5f80fdfea164736f6c634300081c000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1Cct#\xEB<\x14a\0$W_\x80\xFD[4a\x01?W_6`\x03\x19\x01\x12a\x01?W23\x03a\x010W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x15a\x01%W[a\x01\x16W_\x80\x80\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\nZ\x04\xFA=\x15a\x01\x11W=g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xFDW`@Q\x90`\x1F\x81\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\0\xFDW`@R\x81R_` =\x92\x01>[\x15a\0\xEEW_\x80T`\xFF\x19\x16`\x01\x17\x90U\0[cp\xA4\x07\x8F`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[a\0\xDBV[c\x0FE\xB9\x8B`\xE4\x1B_R`\x04_\xFD[P`\xFF_T\x16a\0cV[c\tfP\xC5`\xE2\x1B_R`\x04_\xFD[_\x80\xFD\xFE\xA1dsolcC\0\x08\x1C\0\n",
    );
    /**Custom error with signature `OnlyDirectCalls()` and selector `0x25994314`.
```solidity
error OnlyDirectCalls();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyDirectCalls {}
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
        impl ::core::convert::From<OnlyDirectCalls> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyDirectCalls) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyDirectCalls {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyDirectCalls {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyDirectCalls()";
            const SELECTOR: [u8; 4] = [37u8, 153u8, 67u8, 20u8];
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
    /**Custom error with signature `TStoreAlreadyActivated()` and selector `0xf45b98b0`.
```solidity
error TStoreAlreadyActivated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TStoreAlreadyActivated {}
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
        impl ::core::convert::From<TStoreAlreadyActivated> for UnderlyingRustTuple<'_> {
            fn from(value: TStoreAlreadyActivated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TStoreAlreadyActivated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TStoreAlreadyActivated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TStoreAlreadyActivated()";
            const SELECTOR: [u8; 4] = [244u8, 91u8, 152u8, 176u8];
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
    /**Custom error with signature `TStoreNotSupported()` and selector `0x70a4078f`.
```solidity
error TStoreNotSupported();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TStoreNotSupported {}
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
        impl ::core::convert::From<TStoreNotSupported> for UnderlyingRustTuple<'_> {
            fn from(value: TStoreNotSupported) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TStoreNotSupported {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TStoreNotSupported {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TStoreNotSupported()";
            const SELECTOR: [u8; 4] = [112u8, 164u8, 7u8, 143u8];
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
    /**Custom error with signature `TloadTestContractDeploymentFailed()` and selector `0x2aea5887`.
```solidity
error TloadTestContractDeploymentFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TloadTestContractDeploymentFailed {}
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
        impl ::core::convert::From<TloadTestContractDeploymentFailed>
        for UnderlyingRustTuple<'_> {
            fn from(value: TloadTestContractDeploymentFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for TloadTestContractDeploymentFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TloadTestContractDeploymentFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TloadTestContractDeploymentFailed()";
            const SELECTOR: [u8; 4] = [42u8, 234u8, 88u8, 135u8];
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
    /**Constructor`.
```solidity
constructor();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
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
                ()
            }
        }
    };
    /**Function with signature `__activateTstore()` and selector `0x7423eb3c`.
```solidity
function __activateTstore() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __activateTstoreCall {}
    ///Container type for the return parameters of the [`__activateTstore()`](__activateTstoreCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct __activateTstoreReturn {}
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
            impl ::core::convert::From<__activateTstoreCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: __activateTstoreCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __activateTstoreCall {
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
            impl ::core::convert::From<__activateTstoreReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: __activateTstoreReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __activateTstoreReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __activateTstoreCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __activateTstoreReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__activateTstore()";
            const SELECTOR: [u8; 4] = [116u8, 35u8, 235u8, 60u8];
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
    ///Container for all the [`Tstorish`](self) function calls.
    pub enum TstorishCalls {
        __activateTstore(__activateTstoreCall),
    }
    #[automatically_derived]
    impl TstorishCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[116u8, 35u8, 235u8, 60u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for TstorishCalls {
        const NAME: &'static str = "TstorishCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::__activateTstore(_) => {
                    <__activateTstoreCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<TstorishCalls>] = &[
                {
                    fn __activateTstore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TstorishCalls> {
                        <__activateTstoreCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TstorishCalls::__activateTstore)
                    }
                    __activateTstore
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
                Self::__activateTstore(inner) => {
                    <__activateTstoreCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::__activateTstore(inner) => {
                    <__activateTstoreCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Tstorish`](self) custom errors.
    pub enum TstorishErrors {
        OnlyDirectCalls(OnlyDirectCalls),
        TStoreAlreadyActivated(TStoreAlreadyActivated),
        TStoreNotSupported(TStoreNotSupported),
        TloadTestContractDeploymentFailed(TloadTestContractDeploymentFailed),
    }
    #[automatically_derived]
    impl TstorishErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [37u8, 153u8, 67u8, 20u8],
            [42u8, 234u8, 88u8, 135u8],
            [112u8, 164u8, 7u8, 143u8],
            [244u8, 91u8, 152u8, 176u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for TstorishErrors {
        const NAME: &'static str = "TstorishErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OnlyDirectCalls(_) => {
                    <OnlyDirectCalls as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TStoreAlreadyActivated(_) => {
                    <TStoreAlreadyActivated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TStoreNotSupported(_) => {
                    <TStoreNotSupported as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TloadTestContractDeploymentFailed(_) => {
                    <TloadTestContractDeploymentFailed as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<TstorishErrors>] = &[
                {
                    fn OnlyDirectCalls(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TstorishErrors> {
                        <OnlyDirectCalls as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TstorishErrors::OnlyDirectCalls)
                    }
                    OnlyDirectCalls
                },
                {
                    fn TloadTestContractDeploymentFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TstorishErrors> {
                        <TloadTestContractDeploymentFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TstorishErrors::TloadTestContractDeploymentFailed)
                    }
                    TloadTestContractDeploymentFailed
                },
                {
                    fn TStoreNotSupported(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TstorishErrors> {
                        <TStoreNotSupported as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TstorishErrors::TStoreNotSupported)
                    }
                    TStoreNotSupported
                },
                {
                    fn TStoreAlreadyActivated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<TstorishErrors> {
                        <TStoreAlreadyActivated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(TstorishErrors::TStoreAlreadyActivated)
                    }
                    TStoreAlreadyActivated
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
                Self::OnlyDirectCalls(inner) => {
                    <OnlyDirectCalls as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TStoreAlreadyActivated(inner) => {
                    <TStoreAlreadyActivated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TStoreNotSupported(inner) => {
                    <TStoreNotSupported as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TloadTestContractDeploymentFailed(inner) => {
                    <TloadTestContractDeploymentFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::OnlyDirectCalls(inner) => {
                    <OnlyDirectCalls as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TStoreAlreadyActivated(inner) => {
                    <TStoreAlreadyActivated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TStoreNotSupported(inner) => {
                    <TStoreNotSupported as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TloadTestContractDeploymentFailed(inner) => {
                    <TloadTestContractDeploymentFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Tstorish`](self) contract instance.

See the [wrapper's documentation](`TstorishInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> TstorishInstance<T, P, N> {
        TstorishInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<TstorishInstance<T, P, N>>,
    > {
        TstorishInstance::<T, P, N>::deploy(provider)
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
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        TstorishInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`Tstorish`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Tstorish`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct TstorishInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for TstorishInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("TstorishInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TstorishInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Tstorish`](self) contract instance.

See the [wrapper's documentation](`TstorishInstance`) for more details.*/
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
        ) -> alloy_contract::Result<TstorishInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> TstorishInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> TstorishInstance<T, P, N> {
            TstorishInstance {
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
    > TstorishInstance<T, P, N> {
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
        ///Creates a new call builder for the [`__activateTstore`] function.
        pub fn __activateTstore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, __activateTstoreCall, N> {
            self.call_builder(&__activateTstoreCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > TstorishInstance<T, P, N> {
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