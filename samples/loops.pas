var x, y : integer;

begin
  x := 10;
  y := 5;
  repeat
    x := x - 1;
    y := y + 1;
    write(x);
  until x <> 4;

  while y <> 5 do
    begin
      y := y - 1;
      write(y);
    end;
end.
