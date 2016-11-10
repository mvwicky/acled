<html>
	<head>
		<title>ACLED</title>
		<script defer src="/dart/main.dart" type="application/dart"></script>
		<script defer src="packages/browser/dart.js"></script>
        <link rel="stylesheet" type="text/css" href="/static/styles/normalize.css">
		<link rel="stylesheet" type="text/css" href="/static/styles/main.css">
	</head>
	<body>
        <div class="country-year">
            <h2 id="country-year-header">{{name}}: {{year}}</h2>
            <table id="stats">
                <thead>
                    <tr>
                        <th>Events</th>
                        <th>Fatalities</th>
                        <th>Events/Day</th>
                        <th>Fatalities/Event</th>
                        <th>Most Common Actor</th>
                        <th>Most Common Event Type</th>
                    </tr>
                </thead>
                <tbody>
                    {{#years}}
                        <tr>
                            <td>{{year}}</td>
                            <td>{{fat}}</td>
                            <td>{{epd}}</td>
                            <td>{{fpe}}</td>
                            <td>{{mca}}</td>
                            <td>{{mce}}</td>
                        </tr>
                    {{/years}}
                </tbody>
            </table>
            <br />
            <img class="event-month-graph" />
            <img class="event-day-graph" />
            <table id="events">
                <thead>

                </thead>
                <tbody>

                </tbody>
            </table>
        </div>
	</body>
</html>
