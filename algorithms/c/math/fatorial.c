unsigned long long int Fatorial(unsigned long long int num)
{
    int cont;
    unsigned long long int total = num;
    if (num == 0 || num == 1)
        return 1;
    for (cont = num - 1; cont != 1; cont--)
        total = total * cont;
    return total;
}