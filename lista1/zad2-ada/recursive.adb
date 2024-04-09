package body Recursive is
	-- Funkcja obliczająca wartość n!
	function Factorial(n : Natural) return Natural is
	begin
		if n = 0 then
			return 1;
		else
			return n * Factorial(n - 1);
		end if;
	end Factorial;

	-- Funkcja obliczająca największy wspólny dzielnik dwóch liczb naturalnych
	function GCD(a, b : Natural) return Natural is
	begin
		if b = 0 then
			return a;
		else
			return GCD(b, a mod b);
		end if;
	end GCD;

	-- Funkcja rozwiązująca liniowe równanie diofantyczne ax + by = c na liczbach całkowitych
	function DiophantineEquation(a, b, c : Integer) return DiophantineEquationType is
		x, y: Integer;
		result: DiophantineEquationType;
	begin
		if a = 0 and b = 0 and c /= 0 then
			return (0, 0, False);
		elsif a = 0 then
			x := 0;
			y := c / b;
			return (x, y, True);
		elsif b = 0 then
			x := c / a;
			y := 0;
			return (x, y, True);
		end if;

		result := DiophantineEquation(b, a mod b, c);
		x := result.y;
		y := result.x;
		y := y - (a / b) * x;
		return (x, y, result.valid);
	end DiophantineEquation;

end Recursive;
