<html>
	<head>
		<title>ACLED</title>
		<script defer src="/dart/main.dart" type="application/dart"></script>
		<script defer src="packages/browser/dart.js"></script>
        <link rel="stylesheet" type="text/css" href="/static/styles/normalize.css">
		<link rel="stylesheet" type="text/css" href="/static/styles/main.css">
	</head>
	<body>
        <h1>ACLED.rs</h1>
        <br />
        {{#found}}
        <h2>{{name}}</h2>
        <table>
            <thead>
                <tr>
                    <th>Year</th>
                    <th>Events</th>
                    <th>Fatalities</th>
                    <th>Events/Day</th>
                    <th>Fatalities/Event</th>
                </tr>
            </thead>
            <tbody>
                {{#years}}
                <tr>
                    <td><a href="/{{year}}">{{ year }}</a></td>
                    <td>{{ events }}</td>
                    <td>{{ fatalities }}</td>
                    <td>{{ epd }}</td>
                    <td>{{ fpe }}</td>
                <tr>
                {{/years}}
            </tbody>
        </table>
        {{/found}}

        {{^found}}
            <h3>Country name not found</h3>
        {{/found}}
	</body>
</html>
