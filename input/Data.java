package ml.nocap.utils;

/**
 * Автор: NoCap
 * Дата создания: 05.09.2025
 */
public class SessionData {
    private static String USERNAME = "NoCap";
    private static int UID = 1;
    private static String HWID = "-";
    private static String CLIENT = "Taksa";
    private static String DOMEN = ".pw";

    public static String getUsername() {
        return USERNAME;
    }

    public static int getUid() {
        return UID;
    }

    public static String getHwid() {
        return HWID;
    }

    public static String getClient() {
        return CLIENT;
    }

    public static String getDomen() {
        return CLIENT + DOMEN;
    }
}