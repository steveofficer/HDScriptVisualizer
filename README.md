This is an experimental application that uses Rust->WASM alongside React to perform
parsing HotDocs scripts from a component file and display the relationships between the various components.

This allows people to see how the various components are used within the script.

The project has 3 main parts to it.

1. A script parser and analyzer written in Rust and compiled as a Web Assembly Module. This code is located in `src/native`
2. A Web Worker that runs the Web Assembly Module on a background thread. This code is located in `src/web-worker`
3. A React application that allows a user to open a component file, triggers the Web Worker to run the analyzer, and then renders the
   result as a directed graph.

## Example
The following component file
```xml
<?xml version="1.0" encoding="UTF-8"?>
<hd:componentLibrary xmlns:hd="http://www.hotdocs.com/schemas/component_library/2009" version="12">
	<hd:preferences>
    </hd:preferences>
    <hd:components>
		<hd:text name="Person First Name" warnIfUnanswered="false">
			<hd:prompt>NONE</hd:prompt>
		</hd:text>
        <hd:text name="Person Last Name" warnIfUnanswered="false">
			<hd:prompt>NONE</hd:prompt>
		</hd:text>
        <hd:date name="Person Date of Birth" warnIfUnanswered="false">
			<hd:prompt>NONE</hd:prompt>
		</hd:date>
        <hd:trueFalse name="Eligible for Discount">
            <hd:prompt>NONE</hd:prompt>
        </hd:trueFalse>
        <hd:computation name="CalculateDiscountEligible">
			<hd:script>
                IF AGE(Person Date of Birth) > 60
                    SET Eligible for Discount TO TRUE
                ELSE
                    SET Eligible for Discount TO FALSE
                END IF
            </hd:script>
		</hd:computation>
        <hd:dialog name="Person Details">
			<hd:contents>
				<hd:item name="Person First Name" />
                <hd:item name="Person Last Name" />
                <hd:item name="Person Date of Birth" />
				<hd:item name="Eligible for Discount" />
			</hd:contents>
			<hd:script>
                CalculateDiscountEligible
            </hd:script>
		</hd:dialog>
    </hd:components>
</hd:componentLibrary>
```
produces the following graph ![Screenshot](/images/Example.png)
