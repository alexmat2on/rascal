var x, y;

begin
  x := 10;
  y := 5;
  repeat
    x := x - 1;
    y := y + 1;
    write(x);
  until x <> 4;
  write(y);
end.
