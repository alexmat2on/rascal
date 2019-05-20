var x, y : integer;
var a : array [0..4] of integer;

begin
  x := 10;
  y := 0;

  a[0] := 42;
  a[1] := 41;
  a[2] := 40;
  a[3] := 39;
  a[4] := 38;

  write( a[0] + a[4] );
end.
