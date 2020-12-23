main.jar: main.kt
	kotlinc main.kt -include-runtime -d main.jar

run: main.jar
	java -jar main.jar

clean:
	rm -f main.jar *class

.PHONY: run clean
