package body Iterative is
	-- Implementacja funkcji obliczającej wartość n!
	function Factorial(n : Natural) return Natural is
		result : Natural := 1;
	begin
		for i in 1..n loop
			result := result * i;
		end loop;
		return result;
	end Factorial;

	-- Implementacja funkcji obliczającej największy wspólny dzielnik dwóch liczb naturalnych
	function GCD(a, b : Natural) return Natural is
		a2, b2, temp : Natural;
	begin
		a2 := a;
		b2 := b;

		while b2 /= 0 loop
			temp := b2;
			b2 := a2 mod b2;
			a2 := temp;
		end loop;
		return a2;
	end GCD;

	-- Implementacja funkcji rozwiązującej liniowe równanie diofantyczne ax + by = c
	function DiophantineEquation(a, b, c : Integer) return DiophantineEquationType is
		a2, b2, x1, y1, x2, y2, q, r, temp, x, y : Integer;
	begin
		a2 := a;
		b2 := b;

		x1 := 1;
		y1 := 0;
		x2 := 0;
		y2 := 1;

		while b2 /= 0 loop
			q := a2 / b2;
			r := a2 mod b2;

			temp := x1 - q * x2;
			x1 := x2;
			x2 := temp;

			temp := y1 - q * y2;
			y1 := y2;
			y2 := temp;

			a2 := b2;
			b2:= r;
		end loop;

		if c mod a2 /= 0 then
			return (0, 0, False);
		else
			x := x1 * (c / a2);
			y := y1 * (c / a2);
			return (x, y, True);
		end if;
	end DiophantineEquation;

end Iterative;
