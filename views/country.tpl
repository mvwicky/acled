<html>
	<head>
		<title>ACLED</title>
		<script defer src="/dart/main.dart" type="application/dart"></script>
		<script defer src="packages/browser/dart.js"></script>
		<link rel="stylesheet" type="text/css" href="/static/styles/main.css">
	</head>
	<body>
        {{#found}}
            <h2>{{ name }} Stats</h2><br />
            <table id="indiv_ctry">
                <thead>
                   <tr>
                        <th>Name</th>
                        <th># of Events</th>
                        <th># of Fatalities</th>
                        <!-- 
                        <th>Avg. Events/Year</th>
                        <th>Avg. Fatalities/Event</th>
                        -->
                   </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{{ name }}</td>
                        <td>{{ events }}</td>
                        <td>{{ fatalities }}</td>
                    <tr>    
                </tbody>
            </table>
            {{#years}}
            <br />
            <table>
                <thead>
                    <tr>
                        <th>Year</th>
                        <th># of Events</th>
                        <th># of Fatalities</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>{{ year }}</td>
                        <td>{{ eve }}</td>
                        <td>{{ fat }}</td>
                    <tr>
                </tbody>
            </table>
            {{/years}}
        {{/found}}

        {{^found}}
            <h3>Country name not found</h3>
        {{/found}}
	</body>
</html>
