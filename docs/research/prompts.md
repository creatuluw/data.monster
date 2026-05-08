in docs\research add an ascii node tree with the structure of a re-usable chart we can use anywhere in our app for data analysis and visualization, some requirements:

- click on chart segments to filter the data model
- be able to pick a chart from the library and add it to a page
- have a chart config editor for all atributes and props needed
- use the data model relationships defined in our app the be able to use fields accross different linked tables
- have an expression editor/input in which we can add library metric formulas or udf's
- all elements of a chart should be configurable, but 80% of them should have a default according to best practices
- we also need to find chart frameworks that work very well with our current app and stack, use .agents\skills\web-research\SKILL.md to research a shortlist of chart library candidates which could help us.
- an example of a chart we have used earlier in draft version with rough edges can be found in data.monster\src\routes\+page.svelte, this is in no way good practice but does reveal the intent and output we are after from a users perspective
