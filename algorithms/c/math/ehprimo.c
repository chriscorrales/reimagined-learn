int ehprimo(int n)
{
    int i;
    if (n == 2)
        return TRUE;
    if (n <= 1 || n % 2 == 0)
        return FALSE;

    for (i = 3; i * i <= n; i += 2)
    {
        if (n % i == 0)
            return FALSE;
    }
    return TRUE;
}