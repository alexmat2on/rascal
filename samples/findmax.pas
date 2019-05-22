var max, len, counter : integer;
var a : array [0..4] of integer;

begin
  counter := 1;
  len := 5;

  a[0] := 5;
  a[1] := 3;
  a[2] := 15;
  a[3] := 8;
  a[4] := 14;

  max := a[0];

  while counter < len do
  begin
    if max < a[counter] then
    begin
      max := a[counter];
    end;
    counter := counter + 1;
  end;

  write(max);
end.
