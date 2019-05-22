var iterator,
len, j,
swapIndex,
temp : integer;

var a : array [0..4] of integer;

procedure printArray;
begin
  iterator := 0;
  while iterator < len do
  begin
    write( a[iterator] );
    iterator := iterator + 1;
  end;
end;

procedure swap;
begin
  temp := a[swapIndex];
  a[swapIndex] := a[swapIndex + 1];
  a[swapIndex + 1] := temp;
end;

procedure bubbleSort;
begin
  iterator := 0;
  while iterator < len do
  begin
    j := 0;
    while j < (len - 1) do
    begin
      if a[j] > a[j + 1] then
        begin
          swapIndex := j;
          swap;
        end;

      j := j + 1;
    end;
    iterator := iterator + 1;
  end;
end;

begin
  len := 5;

  a[0] := 5;
  a[1] := 25;
  a[2] := 15;
  a[3] := 8;
  a[4] := 14;

  printArray;
  bubbleSort;

  write(888888888);

  printArray;
end.
