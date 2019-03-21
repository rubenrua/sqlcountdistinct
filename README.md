SQL COUNT DISTINCT
==================

> Very old personal project ported to `rust`.

Show info about your databases executing a count_grouped_by query in all rows of all tables; alias, `BI` for terminal.

```
SELECT `:row` AS G, count(*) AS C FROM `:table` GROUP BY G ORDER BY C DESC;
```

Example output:

```
$ ./dist/sqlcountdistinct 'mysql://root@localhost:3306/employees'

Table: "category" (184)
==================================================================
 *  ID                   int(11)              all different
 *  name                 varchar(255)         almost all different
 *  domain               varchar(255)         'Departamento/Servicio': 107, 'Campo unesco': 24, 'Comunicación': 9, 'Actividades varias': 8, 'Actos institucionales': 8....
 *  description          text                 NULL: 176, 'Farmacia, Medicina, Enfermería, Fisioterapia, Logopedia, Odontología...': 1, 'Estadística, Matemáticas, Biología, Física, Química, Geología...': 1, 'Derecho, Economía, Pedagogía, Periodismo, Psicología, Turismo...': 1, 'Arquitectura, Aeronáutica, Telecomunicaciones, Informática, Industriales...': 1....
 *  code                 int(11)              almost all different
 *  parent_category      varchar(255)         NULL: 179, '': 5
 *  image                varchar(255)         NULL: 179, '/arcamm/images/catlogos/salud.jpg': 1, '/arcamm/images/catlogos/experimentales.jpg': 1, '/arcamm/images/catlogos/sociales.jpg': 1, '/arcamm/images/catlogos/tecnicas.jpg': 1....
 *  icon                 varchar(255)         NULL: 155, '': 24, '/arcamm/images/caticons/rojo.png': 1, '/arcamm/images/caticons/verde.png': 1, '/arcamm/images/caticons/azul.png': 1....
 *  priority             int(2)               0: 179, 5: 1, 4: 1, 1: 1, 2: 1....

```


TODO
-----

* Add more tests.
* Show range when rows are datetime (from: 2011-05-16 01:02:59 to 2017-10-16 12:58:45)
* Not use format! to create queries.
* Add `sqlite` and `postgresql`.
* Use `rayon` to parellel queries.


RELEASE STATIC BINARY
---------------------

```sh
rustup target add x86_64-unknown-linux-musl
make release
```

SEE
---

* https://github.com/dbohdan/structured-text-tools
* https://github.com/lostutils/groupby
