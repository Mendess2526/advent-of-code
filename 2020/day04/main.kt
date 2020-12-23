import java.io.File;
import java.util.HashSet;

fun main() {
    val text = File("input").readText()
    val n = text.splitToSequence("\n\n")
        .map { verifyPassport(it) }
        .filterNotNull()
        .filter { verifyPassportFields(it) } // part 2
        .count()
    println("Number of valid passports: $n")
}

fun verifyPassport(passport: String): HashMap<String, String>? {
    val fields = HashMap<String, String>()
    passport
        .splitToSequence(" ", "\n")
        .map { it.trim() }
        .filter { !it.isEmpty() }
        .forEach {
            val kv = it.split(':')
            fields.put(kv[0], kv[1])
        }
    return if(arrayOf("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid").all { fields.containsKey(it) }) {
        fields
    } else {
        null
    }
}

// part 2
fun verifyPassportFields(passport: HashMap<String, String>): Boolean {
    val validators = hashMapOf<String, (String) -> Boolean?>(
        "byr" to { v -> attempt { v.toInt() }?.let { (1920..2002).contains(it) } },
        "iyr" to { v -> attempt { v.toInt() }?.let { (2010..2020).contains(it) } },
        "eyr" to { v -> attempt { v.toInt() }?.let { (2020..2030).contains(it) } },
        "hgt" to { parseHeight(it) },
        "hcl" to { it.matches(Regex("^#[0-9a-fA-F]{6}$")) },
        "ecl" to { arrayOf("amb", "blu", "brn", "gry", "grn", "hzl", "oth").contains(it) },
        "pid" to { it.matches(Regex("^[0-9]{9}$")) },
    )
    return passport
            .filter { validators[it.key]?.invoke(it.value) ?: false }
            .count() == validators.size
}

fun parseHeight(s: String): Boolean? {
    val i = s.length - 2
    return when (s.slice(i until s.length)) {
        "in" -> attempt { s.slice(0 until i).toInt() }?.let { (59..76).contains(it) }
        "cm" -> attempt { s.slice(0 until i).toInt() }?.let { (150..193).contains(it) }
        else -> false
    }
}

fun <T> attempt(f: () -> T): T? {
    try {
        return f()
    } catch (e: Exception) {
        return null
    }
}
