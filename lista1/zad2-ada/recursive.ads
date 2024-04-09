-- Moduł zawierający funkcje rekursywne
with Ada.Text_IO; use Ada.Text_IO;

package Recursive is
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
			External_Name => "factorial_rec";

	-- Funkcja obliczająca największy wspólny dzielnik dwóch liczb naturalnych
	function GCD(a, b : Natural) return Natural
		with
			Export => True,
			Convention => C,
			External_Name => "gcd_rec";

	-- Funkcja rozwiązująca liniowe równanie diofantyczne ax + by = c na liczbach całkowitych
	function DiophantineEquation(a, b, c : Integer) return DiophantineEquationType
		with
			Export => True,
			Convention => C,
			External_Name => "diophantine_equation_rec";

end Recursive;
