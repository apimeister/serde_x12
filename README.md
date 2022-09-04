# serde_x12

A Serde for ASC X12 EDI documents.

This crate is a work in progress. The required X12 definitions are hosted in the `x12-types` crate. 

## sample code

```rust
use x12_types::v004010::*;

let x = Transmission {
    isa: ISA {
        _01: "00".to_string(),
        _02: "          ".to_string(),
        _03: "00".to_string(),
        _04: "          ".to_string(),
        _05: "ZZ".to_string(),
        _06: "SOURCE         ".to_string(),
        _07: "ZZ".to_string(),
        _08: "TARGET         ".to_string(),
        _09: "220524".to_string(),
        _10: "1120".to_string(),
        _11: "U".to_string(),
        _12: "00401".to_string(),
        _13: "000000001".to_string(),
        _14: "0".to_string(),
        _15: "P".to_string(),
        _16: ">".to_string(),
    },
    functional_group: vec![FunctionalGroup {
        gs: GS {
            _01: "QO".to_string(),
            _02: "SOURCE".to_string(),
            _03: "TARGET".to_string(),
            _04: "20220524".to_string(),
            _05: "1600".to_string(),
            _06: "1".to_string(),
            _07: "X".to_string(),
            _08: "004010".to_string(),
        },
        ...
        ge: GE {
            _01: "1".to_string(),
            _02: "1".to_string(),
        },
    }],
    iea: IEA {
        _01: "1".to_string(),
        _02: "000000001".to_string(),
    },
};
let serialized = serde_x12::to_string(&x).unwrap();
// resulting string
//
// ISA*00*          *00*          *ZZ*SOURCE         *ZZ*TARGET         *220524*1120*U*00401*000000001*0*P*>~
// GS*QO*SOURCE*TARGET*20220524*1600*1*X*004010~
// ....
// GE*1*1~
// IEA*1*000000001~
```