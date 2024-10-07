package com.mark42.numbers;

import org.springframework.shell.standard.ShellComponent;
import org.springframework.shell.standard.ShellMethod;
import org.springframework.shell.standard.ShellOption;

import com.mark42.utils.NumberUtils;

@ShellComponent
public class Numbers {

    @ShellMethod(key = "get-binary", value = "Convert a decimal to Binary")
    public String decimalToBinary(
            @ShellOption(
                    value = {"--n"},
                    defaultValue = "0"
            ) String arg1) {

        // Validate String whether it's a Integer or Not
        if (!NumberUtils.isInteger(arg1)) {
            return "Enter an Integer Idiot 🥱";
        }
        int num = Integer.parseInt(arg1, 10);
        StringBuilder res = new StringBuilder();
        for (; num > 0; num = num / 2) {
            res.append(num % 2);
        }

        return res.reverse().toString();
    }

}