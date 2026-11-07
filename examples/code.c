#define F_CPU 16000000UL
#include <avr/io.h>
#include <util/delay.h>

int main(void) {
    DDRB = 0x3F;                 // PB0..PB5 as outputs

    while (1) {
        for (int j = 0; j < 64; j++) {
            PORTB = j;    // one LED at a time
            _delay_ms(500);
        }
    }
}