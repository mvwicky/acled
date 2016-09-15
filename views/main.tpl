<html>
	<head>
		<title>ACLED</title>
		<script defer src="main.dart" type="application/dart"></script>
		<script defer src="packages/browser/dart.js"></script>
		<link rel="stylesheet" type="text/css" href="main.css">
	</head>
	<body>
        {{#title}}
           <h1> {{t_string}} </h1>
        {{/title}}
		<br />
		<!-- <img class="map" src="africa_map_md.png"> -->
      
        <h3> Country List </h3>
        {{#countries}}
           <b><a href="country/{{ name }}">{{ name }}</a></b> - {{ fatalities }} - {{ events }}<br />
        {{/countries}}
	</body>
</html>
