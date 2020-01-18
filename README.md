periodictable
=========
 [![Build Status](https://travis-ci.com/DomiDre/periodictable.svg?branch=master)](https://travis-ci.com/DomiDre/periodictable)
 [![](http://meritbadge.herokuapp.com/periodictable)](https://crates.io/crates/periodictable)
 
 ``periodictable`` is a library that contains information for the elements such as their mass, density and xray/neutron scattering information.
 
 In its core this crate is a port of the Python library (periodictable of P. Kienzle)[https://github.com/pkienzle/periodictable].

 Basic usage is achieved by initializing the periodic table. Getting the element of interest either by it's atomic number
 or using the symbol() or name() functions of PeriodicTable.
 Then the various properties of the element can be accessed. Refer to docs.rs for more details.


 ```Rust
    use periodictable::PeriodicTable;

    let table = PeriodicTable::new();
    let iron = &table.elements[26];
    let oxygen = &table.symbol("O");
```