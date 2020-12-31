.option norvc

.section text.init

.global _entry
_entry:
spin:
        wfi
        j spin

