# Gist

The PPU has a general working layout, as well as some very specific quirks.

The idea would be to first implement the general behaviour is a relatively straightfoward way (i.e. in a single, modural rendering block), and once this works, fine tune it to be realtively cycle accurate, and to implement the PPU's quirks.


## Registers
https://wiki.nesdev.com/w/index.php/PPU_registers




## Quirks

* The PPU needs some time after power-up to be fully operational. In the meantime, not everything is working (i.e. not all register access from the CPU will work). Details here https://wiki.nesdev.com/w/index.php/PPU_power_up_state