int MDC(int N, int D)
{
    if (D == 0)
        return N;
    else
        return MDC(D, N % D);
}
