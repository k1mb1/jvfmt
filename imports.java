package meqw.eqwe.a     ;

import static io.swagger.v3.oas.annotations.media.Schema.RequiredMode.ROT_REQUIRED;
import static ioa.swagger.v3.oas.annotations.media.Schema.RequiredMode.ROT_REQUIRED;
import java.util.List;

import java.util.Arrays;
import me.an;
import me.am;
import me.aq;


public class SortImports {

    /**
     * Sorts an array of Java import statements lexicographically.
     *
     * @param imports Array of import statements (e.g., "import java.util.List;")
     * @return Sorted array of import statements
     */
    public static String[] sortImports(String[] imports) {
        String[] sorted = Arrays.copyOf(imports, imports.length);
        Arrays.sort(sorted, String::compareTo);
        return sorted;
    }

    public static void main(String[] args) {
        String[] imports = {
            "import java.util.List;",
            "import java.io.File;",
            "import java.util.Map;",
            "import java.util.Collections;"
        };

        String[] sorted = sortImports(imports);

        System.out.println("Sorted imports:");
        for (String imp : sorted) {
            System.out.println(imp);
        }
    }
}