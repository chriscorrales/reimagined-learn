#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#include <ctype.h>
int CodDaRepresentacao (char jogada[]){
	int RepresNumer;
	if(strncmp(jogada,"papel",5)==0)
		RepresNumer=0;
	else
		if(strncmp(jogada,"pedra",5)==0)
			RepresNumer=1;
		else
			if(strncmp(jogada,"ataque",6)==0)
				RepresNumer=2;
	return RepresNumer;
}
int main (){
	char Jog1[8],Jog2[8];
	int N,cont,Jogad1,Jogad2;
	scanf("%d",&N);
	for(cont=1;cont<=N;cont++){
		scanf("%s%s",Jog1,Jog2);
		Jogad1=CodDaRepresentacao(Jog1);
		Jogad2=CodDaRepresentacao(Jog2);
		if(Jogad1!=Jogad2){
			if(Jogad1>Jogad2)
				printf("Jogador 1 venceu\n");
			else
				if(Jogad2>Jogad1)
					printf("Jogador 2 venceu\n");
		}
		else{
			if(Jogad1==0)
				printf("Ambos venceram\n");
			else
				if(Jogad1==1)
					printf("Sem ganhador\n");
				else
					if(Jogad1==2)
						printf("Aniquilacao mutua\n");
		}
	}

	return 0;
}