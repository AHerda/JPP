-- Moduł zawierający trzy funkcje matematyczne
with Ada.Text_IO; use Ada.Text_IO;

package Iterative is
	-- Typ reprezentujący równania diofantycznego
	type DiophantineEquationType is record
		x, y : Integer;
		valid: Boolean;
	end record;

	-- Funkcja obliczająca wartość n!
	function Factorial(n : Natural) return Natural
		with
			Export => True,
			Convention => C,
			External_Name => "factorial_iter";

	-- Funkcja obliczająca największy wspólny dzielnik dwóch liczb naturalnych
	function GCD(a, b : Natural) return Natural
		with
			Export => True,
			Convention => C,
			External_Name => "gcd_iter";

	-- Funkcja rozwiązująca liniowe równanie diofantyczne ax + by = c
	function DiophantineEquation(a, b, c : Integer) return DiophantineEquationType
		with
			Export => True,
			Convention => C,
			External_Name => "diophantine_equation_iter";

end Iterative;
