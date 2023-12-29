#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>

#define TRUE 1
#define FALSE -1
#define MAX_VET 100001

struct regBoardAmount
{
  int size, amount;
};
typedef struct regBoardAmount BoardAmount;

int comp(const void *, const void *);

int arr_find_index(unsigned int size, BoardAmount boards[], unsigned int index, unsigned int length)
{
  int i;

  if (length == 0)
  {
    return FALSE;
  }

  for (i = index; i < length; i++)
  {
    if (boards[i].size == size)
    {
      break;
    }
  }

  return (boards[i].size == size) ? i-- : FALSE;
}

int calc_short_board(BoardAmount boards[], unsigned int width_board, unsigned int height_area, unsigned width_area, unsigned int size_array)
{
  unsigned int total_lines, i, j, total_board_line, rest, first_index_with_boards;
  int stop_loop = FALSE, total_boards = 0;
  BoardAmount temp[MAX_VET];
  BoardAmount chavePesq;
  BoardAmount *objectSearch;

  memset(temp, 0, sizeof(temp));
  memcpy(temp, boards, sizeof(BoardAmount) * size_array);

  if ((height_area * 100) % width_board > 0)
  {
    return -1;
  }

  total_lines = (height_area * 100) / width_board;
  first_index_with_boards = 0;

  for (i = 0; i < total_lines; i++)
  {
    rest = width_area;
    total_board_line = 0;
    if (stop_loop == TRUE)
    {
      break;
    }

    for (j = first_index_with_boards; rest != 0 && j < size_array; j++)
    {
      if (temp[j].amount == 0 && j == size_array - 1)
      {
        total_boards = FALSE;
        stop_loop = TRUE;
        break;
      }

      if (temp[j].amount == 0 || temp[j].size > rest)
      {
        first_index_with_boards = j + 1;
        continue;
      }

      rest = rest - temp[j].size;
      temp[j].amount--;

      if (rest > 0)
      {
        chavePesq.size = rest;
        objectSearch = (struct regBoardAmount *)bsearch(&chavePesq, temp, size_array, sizeof(struct regBoardAmount), comp);

        // find_index = arr_find_index(rest, temp, j);
        if (j == size_array - 1 && objectSearch == NULL)
        {
          total_boards = FALSE;
          stop_loop = TRUE;
          break;
        }

        if (objectSearch == NULL || objectSearch->amount == 0)
        {
          temp[j].amount = 0;
          i--;
          break;
        }

        if (objectSearch != NULL)
        {
          total_board_line = total_board_line + 2;
          objectSearch->amount--;
          break;
        }
      }
      else
      {
        total_board_line++;
      }
    }

    total_boards = total_boards + total_board_line;
  }

  return (total_boards == 0) ? FALSE : total_boards;
};

int main()
{

  unsigned int index, iterator, width_area, height_area, width_board, boards_amount, board_size;
  int find_index, total_boards_height, total_boards_width;
  BoardAmount boards[MAX_VET];

  while (TRUE)
  {
    scanf("%u%u", &width_area, &height_area);

    if (width_area == 0 && height_area == 0)
    {
      break;
    }

    scanf("%u%u", &width_board, &boards_amount);

    memset(boards, 0, sizeof(boards));

    index = 0;
    for (iterator = 0; iterator < boards_amount; iterator++)
    {
      scanf("%u", &board_size);

      if (board_size > width_area && board_size > height_area)
      {
        continue;
      }
      find_index = arr_find_index(board_size, boards, 0, index);

      if (iterator == 0 || find_index == FALSE)
      {
        boards[index].amount += 1;
        boards[index].size = board_size;
        index++;
        continue;
      }

      boards[find_index].amount += 1;
    }

    qsort(boards, index, sizeof(struct regBoardAmount), comp);

    total_boards_height = calc_short_board(boards, width_board, height_area, width_area, index);

    total_boards_width = calc_short_board(boards, width_board, width_area, height_area, index);

    if (total_boards_height == FALSE && total_boards_width == FALSE)
    {
      printf("impossivel\n");
      continue;
    }
    else if (total_boards_height == FALSE && total_boards_width != FALSE)
    {
      printf("%u\n", total_boards_width);
    }
    else if (total_boards_height != FALSE && total_boards_width == FALSE)
    {
      printf("%u\n", total_boards_height);
    }
    else if (total_boards_height < total_boards_width)
    {
      printf("%u\n", total_boards_height);
    }
    else
    {
      printf("%u\n", total_boards_width);
    }
  }

  return 0;
}

int comp(const void *p1, const void *p2)
{
  struct regBoardAmount *i = (struct regBoardAmount *)p1, *j = (struct regBoardAmount *)p2;
  if (i->size > j->size)
    return -1;

  else if (i->size == j->size)
    return 0;
  else
    return 1;
}