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
                        <th>Date</th>
                        <th>Type</th>
                        <th>Fatalities</th>
                        <th>Actor 1</th>
                        <th>Actor 2</th>
                        <th>Location</th>
                    </tr>
                </thead>
                <tbody>
                    {{#eve_vec}}
                        <tr>
                            <td>{{ event_date }}</td>
                            <td>{{ event_type }}</td>
                            <td>{{ fatalities }}</td>
                            <td>{{ actor_1 }}</td>
                            <td>{{ actor_2 }}</td>
                            <td>{{ location }}</td>
                        </tr>
                    {{/eve_vec}}
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
