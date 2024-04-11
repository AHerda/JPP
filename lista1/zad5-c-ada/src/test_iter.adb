with Ada.Text_IO; use Ada.Text_IO;
with Interfaces.C; use Interfaces.C;
with x86_64_linux_gnu_bits_types_h; use x86_64_linux_gnu_bits_types_h;
with lib_iter_h; use lib_iter_h;

procedure Test_Iter is
	result_iter : ISolution;
begin
	Put_Line(" === Iteracyjna implementacja funkcji matematycznych ===");
	-- Obliczanie wartości 5!
	Put_Line(" - 5! = " & uu_uint64_t'Image(lib_iter_h.IFactorial(5)));

	-- Obliczanie największego wspólnego dzielnika liczb 24 i 36
	Put_Line(" - NWD(24, 36) = " & uu_uint64_t'Image(lib_iter_h.IGcd(24, 36)));

	-- Rozwiązywanie równania diofantycznego 3x + 5y = -3
	result_iter := lib_iter_h.IDiophantine(3, 5, -3);
	if result_iter.valid then
		Put_Line(" - Rozwiązanie równania 3x + 5y = -3: x = " & uu_int64_t'Image(result_iter.x) & ", y = " & uu_int64_t'Image(result_iter.y));
	else
		Put_Line(" - Rozwiązanie równania 3x + 5y = -3 nie istnieje");
	end if;
end Test_Iter;
