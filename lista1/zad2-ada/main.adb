-- Przykładowe użycie zaimplementowanych funkcji
with Ada.Text_IO; use Ada.Text_IO;
with Iterative; use Iterative;
with Recursive; use Recursive;

procedure Main is
	x, y : Integer;
begin
	Put_Line(" === Iteracyjna implementacja funkcji matematycznych ===");
	-- Obliczanie wartości 5!
	Put_Line(" - 5! = " & Natural'Image(Iterative.Factorial(5)));

	-- Obliczanie największego wspólnego dzielnika liczb 24 i 36
	Put_Line(" - NWD(24, 36) = " & Natural'Image(Iterative.GCD(24, 36)));

	-- Rozwiązywanie równania diofantycznego 3x + 5y = -3
	if Iterative.DiophantineEquation(3, 5, -3, x, y) then
		Put_Line(" - Rozwiązanie równania 3x + 5y = -3: x = " & Integer'Image(x) & ", y = " & Integer'Image(y));
	else
		Put_Line(" - Rozwiązanie równania 3x + 5y = -3 nie istnieje");
	end if;

	Put_Line(" === Rekurencyjna implementacja funkcji matematycznych ===");
	-- Obliczanie wartości 5!
	Put_Line(" - 5! = " & Natural'Image(Recursive.Factorial(5)));

	-- Obliczanie największego wspólnego dzielnika liczb 24 i 36
	Put_Line(" - NWD(24, 36) = " & Natural'Image(Recursive.GCD(24, 36)));

	-- Rozwiązywanie równania diofantycznego 3x + 5y = -3
	if Recursive.DiophantineEquation(3, 5, -3, x, y) then
		Put_Line(" - Rozwiązanie równania 3x + 5y = -3: x = " & Integer'Image(x) & ", y = " & Integer'Image(y));
	else
		Put_Line(" - Rozwiązanie równania 3x + 5y = -3 nie istnieje");
	end if;
end Main;
