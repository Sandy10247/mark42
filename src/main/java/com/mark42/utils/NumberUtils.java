package com.mark42.utils;

import java.util.regex.Pattern;


public class NumberUtils {

    public static boolean isInteger(final CharSequence cs) {
        return  Pattern.compile("\\d+").matcher(cs).matches();
    }
}