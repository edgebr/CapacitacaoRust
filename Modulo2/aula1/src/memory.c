/**
 * @file memory.c
 * @author Matheus T. dos Santos (matheus.santos@edge.ufal.br)
 * @brief
 * @version 0.1
 * @date 15/02/2023
 *
 * @copyright Copyright (c) 2023
 *
 */
#include <stdio.h>
#include <stdint.h>

void print_out(uint8_t *list, size_t list_len) {
    for (int i = 0; i < list_len; ++i) {
        printf("%d\n", list[i]);
    }
}

int main() {
    uint8_t item[] = {1, 2, 3};

    print_out(item, sizeof(item));

    return 0;
}

void print_last() {
    uint8_t item[] = {1, 2, 3};

    uint8_t last_item = item[sizeof(item) - 1];

    item[sizeof(item) - 1] = 0;

    printf("%d\n", last_item);
}

void reverse_and_print(uint8_t *list, size_t len) {
    uint8_t temp;

    for (int i = 0; i < len / 2; ++i) {
        temp = list[len - i - 1];
        list[len - i - 1] = list[i];
        list[i] = temp;
    }

    for (int i = 0; i < len; ++i) {
        printf("%d\n", list[i]);
    }
}

void reverse_and_print2(uint8_t *list, size_t len) {
    for (int i = 0; i < len; ++i) {
        printf("%d\n", list[len - i - 1]);
    }
}
