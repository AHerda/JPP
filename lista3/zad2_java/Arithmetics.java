package JPP.lista3.zad2_java;

public interface Arithmetics {
	Arithmetics mul(Arithmetics other);
	Arithmetics div(Arithmetics other);
	Arithmetics fromLong(long value);
}
