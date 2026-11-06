#include <avr/io.h>
#include <util/delay.h>

#ifndef F_CPU 
#define F_CPU 16000000
#endif

int main(void) {
    DDRB |= (1 << PB5);          // PB5 (LED de l'Arduino Uno) en sortie

    while (1) {
        PORTB ^= (1 << PB5);     // inverse l'état de la broche
        _delay_ms(100);
    }
}