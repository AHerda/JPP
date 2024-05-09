package JPP.lista2.zad2_java;

public class Main {
    public static void main(String[] args) {
        GF a = new GF(1234560);
        GF b = new GF(10);

        System.out.println("a = " + a);
        System.out.println("b = " + b);

        GF c = a.add(b);
        System.out.println("a + b = " + c);
        c = a.sub(b);
        System.out.println("a - b = " + c);
        c = a.mul(b);
        System.out.println("a * b = " + c);
        c = a.div(b);
        System.out.println("a / b = " + c);

        System.out.println("a == b <=> " + a.eq(b));
        System.out.println("a != b <=> " + a.neq(b));
        System.out.println("a < b <=> " + (a.lt(b)));
        System.out.println("a > b <=> " + (a.gt(b)));
        System.out.println("a <= b <=> " + (a.le(b)));
        System.out.println("a >= b <=> " + (a.ge(b)));

        System.out.println("\nCharacteristic od a = " + GF.characteristic());
    }
}
