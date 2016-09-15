<html>
	<head>
		<title>ACLED</title>
		<script defer src="main.dart" type="application/dart"></script>
		<script defer src="packages/browser/dart.js"></script>
		<link rel="stylesheet" type="text/css" href="main.css">
	</head>
	<body>
        {{#found}}
            <h2>{{ name }} Stats</h2><br />
            Number of Events: {{ events }} <br />
            Number of Fatalities: {{ fatalities }} <br />
        {{/found}}

        {{^found}}
            <h3>Country name not found</h3>
        {{/found}}
	</body>
</html>
