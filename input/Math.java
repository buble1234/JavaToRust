package ml.nocap.utils;

public class Check {

    public static void main(String[] args) {
        int a = 10;
        int b = 5;

        int sum = a + b;
        System.out.println("Сумма " + a + " и " + b + " = " + sum);

        int product = a * b;
        System.out.println("Произведение " + a + " и " + b + " = " + product);

        int number = 8;
        if (number % 2 == 0) {
            System.out.println("Число " + number + " четное");
        } else {
            System.out.println("Число " + number + " нечетное");
        }
    }
}