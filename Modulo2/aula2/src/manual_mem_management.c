/**
 * @file manual_mem_management.c
 * @author Matheus T. dos Santos (matheus.santos@edge.ufal.br)
 * @brief
 * @version 0.1
 * @date 13/02/2023
 *
 * @copyright Copyright (c) 2023
 *
 */
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

char *digits_to_string(uint8_t *digit_list, size_t digit_list_len) {
    char *string = (char *) malloc(digit_list_len);

    for (int i = 0; i < digit_list_len; ++i) {
        string[i] = digit_list[i] + '0';
    }

    return string;
}

int main() {
    uint8_t digits[5] = {1, 2, 3, 4, 5};
    char *digits_str = digits_to_string(digits, sizeof(digits));

    printf("Digits: %s\n", digits_str);

    /* Memory Leak aqui! Faltou dar o free() no buffer 'digits_str' */

    return 0;
}
