// only-cdb
// ignore-tidy-linelength
// compile-flags:-g

// cdb-command: g

// Note: The natvis used to visualize niche-layout enums don't work correctly in cdb
//       so the best we can do is to make sure we are generating the right debuginfo

// cdb-command: dx -r2 a,!
// cdb-check:a,!              [Type: _enum<core::option::Option<_enum<msvc_pretty_enums::CStyleEnum>>, 2, 16, Some>]
// cdb-check:    [+0x000] dataful_variant  [Type: _enum<core::option::Option<_enum<msvc_pretty_enums::CStyleEnum>>, 2, 16, Some>::Some]
// cdb-check:        [+0x000] __0              : Low (0x2) [Type: msvc_pretty_enums::CStyleEnum]
// cdb-check:    [+0x000] discriminant$    [Type: _enum<core::option::Option<_enum<msvc_pretty_enums::CStyleEnum>>, 2, 16, Some>::discriminant$]
// cdb-check:        [+0x000] discriminant     : 0x2 [Type: _enum<core::option::Option<_enum<msvc_pretty_enums::CStyleEnum>>, 2, 16, Some>::tag$]

// cdb-command: dx -r2 b,!
// cdb-check:b,!              [Type: _enum<core::option::Option<_enum<msvc_pretty_enums::CStyleEnum>>, 2, 16, Some>]
// cdb-check:    [+0x000] dataful_variant  [Type: _enum<core::option::Option<_enum<msvc_pretty_enums::CStyleEnum>>, 2, 16, Some>::Some]
// cdb-check:        [+0x000] __0              : 0x11 [Type: msvc_pretty_enums::CStyleEnum]
// cdb-check:    [+0x000] discriminant$    [Type: _enum<core::option::Option<_enum<msvc_pretty_enums::CStyleEnum>>, 2, 16, Some>::discriminant$]
// cdb-check:        [+0x000] discriminant     : None (0x11) [Type: _enum<core::option::Option<_enum<msvc_pretty_enums::CStyleEnum>>, 2, 16, Some>::tag$]

// cdb-command: dx -r2 c,!
// cdb-check:c,!              [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
// cdb-check:    [+0x000] dataful_variant  [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
// cdb-check:        [+0x000] my_data          : 0x11 [Type: msvc_pretty_enums::CStyleEnum]
// cdb-check:    [+0x000] discriminant$    [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::discriminant$]
// cdb-check:        [+0x000] discriminant     : Tag1 (0x11) [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::tag$]

// cdb-command: dx -r2 d,!
// cdb-check:d,!              [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
// cdb-check:    [+0x000] dataful_variant  [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
// cdb-check:        [+0x000] my_data          : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]
// cdb-check:    [+0x000] discriminant$    [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::discriminant$]
// cdb-check:        [+0x000] discriminant     : 0x10 [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::tag$]

// cdb-command: dx -r2 e,!
// cdb-check:e,!              [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>]
// cdb-check:    [+0x000] dataful_variant  [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::Data]
// cdb-check:        [+0x000] my_data          : 0x13 [Type: msvc_pretty_enums::CStyleEnum]
// cdb-check:    [+0x000] discriminant$    [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::discriminant$]
// cdb-check:        [+0x000] discriminant     : Tag2 (0x13) [Type: _enum<msvc_pretty_enums::NicheLayoutEnum, 2, 16, Data>::tag$]

// cdb-command: dx -r2 f,!
// cdb-check:f,!              [Type: _enum<core::option::Option<u32*>, 1, [...], Some>]
// cdb-check:    [+0x000] dataful_variant  [Type: _enum<core::option::Option<u32*>, 1, [...], Some>::Some]
// cdb-check:        [+0x000] __0              : 0x[...] : 0x1 [Type: unsigned int *]
// cdb-check:    [+0x000] discriminant$    [Type: _enum<core::option::Option<u32*>, 1, [...], Some>::discriminant$]
// cdb-check:        [+0x000] discriminant     : 0x[...] [Type: _enum<core::option::Option<u32*>, 1, [...], Some>::tag$]

// cdb-command: dx -r2 g,!
// cdb-check:g,!              [Type: _enum<core::option::Option<u32*>, 1, [...], Some>]
// cdb-check:    [+0x000] dataful_variant  [Type: _enum<core::option::Option<u32*>, 1, [...], Some>::Some]
// cdb-check:        [+0x000] __0              : 0x0 [Type: unsigned int *]
// cdb-check:    [+0x000] discriminant$    [Type: _enum<core::option::Option<u32*>, 1, [...], Some>::discriminant$]
// cdb-check:        [+0x000] discriminant     : None (0x0) [Type: _enum<core::option::Option<u32*>, 1, [...], Some>::tag$]

// cdb-command: dx h
// cdb-check:h                : Some [Type: _enum<core::option::Option<u32>>]
// cdb-check:    [+0x000] variant$         : Some (0x1) [Type: core::option::Option]
// cdb-check:    [+0x004] __0              : 0xc [Type: unsigned int]

// cdb-command: dx i
// cdb-check:i                : None [Type: _enum<core::option::Option<u32>>]
// cdb-check:    [+0x000] variant$         : None (0x0) [Type: core::option::Option]

// cdb-command: dx j
// cdb-check:j                : High (0x10) [Type: msvc_pretty_enums::CStyleEnum]

// cdb-command: dx -r2 k,!
// cdb-check:k,!              [Type: _enum<core::option::Option<alloc::string::String>, 1, [...], Some>]
// cdb-check:    [+0x000] dataful_variant  [Type: _enum<core::option::Option<alloc::string::String>, 1, [...], Some>::Some]
// cdb-check:        [+0x000] __0              [Type: alloc::string::String]
// cdb-check:    [+0x000] discriminant$    [Type: _enum<core::option::Option<alloc::string::String>, 1, [...], Some>::discriminant$]
// cdb-check:        [+0x000] discriminant     : 0x[...] [Type: _enum<core::option::Option<alloc::string::String>, 1, [...], Some>::tag$]

pub enum CStyleEnum {
    Low = 2,
    High = 16,
}

pub enum NicheLayoutEnum {
    Tag1,
    Data { my_data: CStyleEnum },
    Tag2,
}

fn main() {
    let a = Some(CStyleEnum::Low);
    let b = Option::<CStyleEnum>::None;
    let c = NicheLayoutEnum::Tag1;
    let d = NicheLayoutEnum::Data { my_data: CStyleEnum::High };
    let e = NicheLayoutEnum::Tag2;
    let f = Some(&1u32);
    let g = Option::<&'static u32>::None;
    let h = Some(12u32);
    let i = Option::<u32>::None;
    let j = CStyleEnum::High;
    let k = Some("IAMA optional string!".to_string());

    zzz(); // #break
}

fn zzz() { () }
