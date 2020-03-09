--
-- PostgreSQL database dump
--

-- Dumped from database version 10.12 (Ubuntu 10.12-0ubuntu0.18.04.1)
-- Dumped by pg_dump version 10.12 (Ubuntu 10.12-0ubuntu0.18.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner:
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner:
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: autoresponse_phrases; Type: TABLE; Schema: public; Owner: ross
--

CREATE TABLE public.autoresponse_phrases (
    id integer NOT NULL,
    input text NOT NULL,
    rule_type character varying(255) NOT NULL,
    reply_to boolean NOT NULL,
    output text[] NOT NULL
);

--
-- Name: autoresponse_phrases_id_seq; Type: SEQUENCE; Schema: public; Owner: ross
--

CREATE SEQUENCE public.autoresponse_phrases_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: autoresponse_phrases_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: ross
--

ALTER SEQUENCE public.autoresponse_phrases_id_seq OWNED BY public.autoresponse_phrases.id;


--
-- Name: feeds; Type: TABLE; Schema: public; Owner: ross
--

CREATE TABLE public.feeds (
    id integer NOT NULL,
    url character varying(255) NOT NULL,
    kind character varying(100) NOT NULL,
    timeout integer,
    last_update timestamp without time zone,
    last_entry text
);

--
-- Name: feeds_id_seq; Type: SEQUENCE; Schema: public; Owner: ross
--

CREATE SEQUENCE public.feeds_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: feeds_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: ross
--

ALTER SEQUENCE public.feeds_id_seq OWNED BY public.feeds.id;


--
-- Name: greetings; Type: TABLE; Schema: public; Owner: ross
--

CREATE TABLE public.greetings (
    id integer NOT NULL,
    text text NOT NULL
);

--
-- Name: greetings_id_seq; Type: SEQUENCE; Schema: public; Owner: ross
--

CREATE SEQUENCE public.greetings_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: greetings_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: ross
--

ALTER SEQUENCE public.greetings_id_seq OWNED BY public.greetings.id;


--
-- Name: schedule; Type: TABLE; Schema: public; Owner: ross
--

CREATE TABLE public.schedule (
    id integer NOT NULL,
    day character varying(100) NOT NULL,
    "time" time without time zone NOT NULL,
    messages text[] NOT NULL
);

--
-- Name: schedule_id_seq; Type: SEQUENCE; Schema: public; Owner: ross
--

CREATE SEQUENCE public.schedule_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: schedule_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: ross
--

ALTER SEQUENCE public.schedule_id_seq OWNED BY public.schedule.id;


--
-- Name: shippering_phrases; Type: TABLE; Schema: public; Owner: ross
--

CREATE TABLE public.shippering_phrases (
    id integer NOT NULL,
    template text
);

--
-- Name: shippering_phrases_id_seq; Type: SEQUENCE; Schema: public; Owner: ross
--

CREATE SEQUENCE public.shippering_phrases_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: shippering_phrases_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: ross
--

ALTER SEQUENCE public.shippering_phrases_id_seq OWNED BY public.shippering_phrases.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: ross
--

CREATE TABLE public.users (
    id bigint NOT NULL,
    first_name character varying(255) NOT NULL,
    last_name character varying(255),
    username character varying(255)
);

--
-- Name: autoresponse_phrases id; Type: DEFAULT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.autoresponse_phrases ALTER COLUMN id SET DEFAULT nextval('public.autoresponse_phrases_id_seq'::regclass);


--
-- Name: feeds id; Type: DEFAULT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.feeds ALTER COLUMN id SET DEFAULT nextval('public.feeds_id_seq'::regclass);


--
-- Name: greetings id; Type: DEFAULT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.greetings ALTER COLUMN id SET DEFAULT nextval('public.greetings_id_seq'::regclass);


--
-- Name: schedule id; Type: DEFAULT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.schedule ALTER COLUMN id SET DEFAULT nextval('public.schedule_id_seq'::regclass);


--
-- Name: shippering_phrases id; Type: DEFAULT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.shippering_phrases ALTER COLUMN id SET DEFAULT nextval('public.shippering_phrases_id_seq'::regclass);


--
-- Data for Name: autoresponse_phrases; Type: TABLE DATA; Schema: public; Owner: ross
--

COPY public.autoresponse_phrases (id, input, rule_type, reply_to, output) FROM stdin;
1	nlinker	contains	t	{t.me/rustjerk/317962}
2	пельмени	contains	t	{"Жаренные пельмени - топ!"}
3	/0	equals	t	{"<pre>thread 'rustjerk' panicked at 'attempt to divide by zero'</pre>"}
4	/thread	equals	t	{"<pre>thread 'rustjerk' panicked at 'attempt to stop thread'</pre>"}
5	define go	equals	f	{"\\"Читая твой пост я, наконец, понял философию дизайна и целевую аудиторию go. Это язык для слабых программистов. Это самое политкорректное выражение которое я смог подобрать. У go даже эмблема со зверюшкой, у которой явно синдром дауна. По всей видимости, авторы получили указание от начальства такого вида: Мы тут набрали кучу случайных людей по объявлениям и квотам на негров, геев, феминисток итп. Уволить мы их не можем, ибо SJW обвинят нас в расизме, гомофобии и что там ещё популярно у SJW. Нам нужен язык, который даже эти могут использовать. Разумеется, никто никогда в этом не сознается. Но других объяснений этому языку у меня нет.\\" (c) rsdn"}
6	define jerk	equals	f	{https://telegra.ph/Dzherk-12-28}
8	алло	equals	f	{"<b>YOBA ETO TI?</b>"}
9	дженерики	equals	t	{"Ну, может быть, и хорошо. Пока что выглядит, как чисто сисадминский язык :) Дженерики, по-хорошему, это задача для искусственного интеллекта, а у меня его нету :)","Ну, может быть, и хорошо. Пока что выглядит, как чисто сисадминский язык :) Дженерики, по-хорошему, это задача для интеллекта, а у меня его нету :)"}
10	минуту	equals	t	{"Специалист ответил на все ваши вопросы, продолжить консультацию?"}
11	митап	equals	t	{"Ты хочешь митапы как у растджерка, но ты делаешь это без уважения к культуре джерка, ты даже не пишешь в джерке"}
12	охуенный джерк	equals	t	{"Блять, какой охуенный джерк, ржем с остальных чатов, а ведь кто-нибудь джеркает на нас"}
13	парк горького очень преобразился за последние годы	equals	t	{"Прикинь, едешь такой на конячке по городу, обустроенному нашим замечательным мэром. Спасибо большое, Сергею Собянину! И тут конь превращается в адскую машину, но внезапно мешок переполняется и с краёв удобряет прекрасный город!"}
14	паста про раст	equals	t	{"Завтра ищешь в интернете книжку The Rust Programming Language. Похуй если ничего не поймешь. Затем идешь на rust-lang.org и изучаешь стандартную библиотеку от корки до корки. Потом зубришь, именно, сука, вызубриваешь конфиг rustfmt, чтобы от зубов отскакивало. Когда напишешь свою первую имиджборду, по пути изучив верстку на html+css, скачиваешь и изучаешь любой растовый асинхронный вебсервер, рекомендую hyper или actix-web. Как переделаешь имиджборду, чтобы выдавала по крайней мере 5 тысяч запросов в секунду, можешь идти дальше - тебя ждет увлекательный мир хайлоада. Tokio, сверхбыстрые асинхронные key-value хранилища, MapReduce. Отсос хиккующих выблядков / просто неудачников типа инва2004 или сисярп/джава-хуесосов типа Жуковского, которые сосут хуй по жизни не заставит себя ждать, и уже через пол года ты будешь получать такие суммы, что любая баба будет течь при одном упоминании твоей зарплаты."}
15	помянем	equals	t	{https://www.youtube.com/watch?v=7Fmuq91CCm4}
16	раст заебись тема	equals	t	{https://telegra.ph/Rast-zaebis-tema-01-04}
17	Пообедала ли ты на ночь, Анастейша?	equals	t	{@anastaysha164}
18	ты тупой	equals	f	{"конечно, ведь ты строишь из себя охуительного мудреца и скрываешься за стеной плохо связанных философских рассуждений тебе кажется, что тебя начнут считать умным, но по факту просто срешь помойкой из слов, вместо того, чтобы структурировать и доносить мысль до собеседника, никто в джерке больше на это не ведется твои рассуждения могли приводить с ахуительным флеймам, но они больше не приводят, потому что мы устали плескаться в болоте стен текста из дистиллированной воды, не заполненной смыслом, демагогия должна иметь под собой идею но ты все еще можешь написать четыре тома размышлений, и возможно, когда ты умрешь никому не нужным в старости, о тебе вспомнят и твои книги будут читать на философских факультетах. НО ЗАЧЕМ, $USERNAME? ЗАЧЕМ?"}
20	(?i)(net t[iy]|no u)	matches	t	{"<b>NET TY</b>","<b>POSHEL NAHUY</b>","<b>I TY TOZHE IDI NAHUY</b>",<b>YEDINIMSYA?</b>}
21	(?i)php|пхп	matches	f	{@PandaThePanda,https://youtu.be/JeuJvO2l-Uk}
22	^(вим|[иеэ]м[аэ]кс|vim|emacs)$	matches	t	{"с вимом хоть понятно за что его любить, там необычный подход к редактированию (мне не нравится, но ничего удивительного в том, что кому-то больше именно это понравится). Вот любовь к имаксу для меня совершенно непонятна, ведь это обычный редактор, просто старый, страшный, убогий и тормозной"}
23	(?i)^выбери (раст|rust)$	matches	t	{"Выбери Rust. Выбери разработку. Выбери билд. Выбери джерк. Выбери трэйт баунды на 30 строк. Выбери генераторы, тип суммы, футуры и итераторы. Выбери компиляцию по 5 часов, 2000+ зависмостей и мемори сэйфити. Выбери карго и аккуратно заворачивай ансейфы. Выбери свой первый пет проджект с <code>assert_eq!(2+2, 4)</code>. Выбери нико, центрила и крихтона. Выбери компиляцию в васми и 30 с хуем других таргетов. Выбери клиппи и карго чек на вотче. В свой выходной выбери отправить пр в райс юай и смотреть отупляющий доклад про зиро кост абстракшенс. Забивай голову ньютайпами. Выбери загнивание на обочине полукилометровых ошибок от токио и со стыдом вспоминай подонков, которых ты оставил продолжать писать иф ерор не равно нил. Выбери своё будущее. Выбери Rust. Но зачем мне всё это? Я не стал выбирать Rust, я выбрал кое-что другое. Почему? Да ни почему. Какие могут быть \\"почему\\", когда есть Go?"}
24	(?i)д(е|э)вопс	matches	t	{"<a href=\\"tg://user?id=138098452\\">АРТЁМ</a>"}
25	(i?)гендер	matches	t	{"Есть только два гендера"}
26	(?i)^жоск[оа]$	matches	t	{"Хигашиката Джоске","Хигасиката Дзёсуке","Higashikata Josuke",Жоске}
27	(?i)кадыров	matches	f	{"<b>АХМАТ СИЛА!</b>"}
28	(?i)кто такой жуковский	matches	f	{"тот, кому неприятен жерк и он создал раст оффтопик и который раньше бесил Дениса в гиттере а теперь освоился и бесит в телеграме"}
29	(?i)н[еи]\\s?нуж[еиы]?н[оыа]?	matches	f	{"это ты не нужен"}
30	(?i)не понимаешь сути|^суть$	matches	t	{http://telegra.ph/TY-NE-PONIMAESH-SUTI-06-22}
31	(?i)^(перформанс|перфоманс|perfomance|performance|производительность)$	matches	t	{http://telegra.ph/PERFORMANCE-06-22}
32	помоги(те)?	matches	f	{"Ты приходишь в джерк и просишь нас о помощи, но ты делаешь это без кислоты"}
33	(?i)#?прочитал(весь)?джерк(нахуй)?	matches	f	{"Почему, почему? Во имя чего? Что Вы делаете? Зачем Вы читаете весь джерк? Зачем Вы продолжаете? Неужели Вы верите в какую-то миссию или Вам просто страшно от одиночества? Так в чем же миссия, может быть Вы откроете? Это свобода, правда, может быть познание, или Вы боритесь за токсичность? Срачи, причуды общения. Хрупкие логические цепочки слабого человека, отчаянно пытающегося оправдать свою прокрастинацию: бесцельную и бессмысленную. Но срачи, как и джерк, столь же искуственны. Только человек может выдумать скучное и безжизненное понятие \\"феминизм\\". Вам пора увидеть, увидеть и понять: Вы не можете прочитать весь джерк, продолжать читать бессмысленно! Почему, почему Вы упорствуете?","<a href=\\"tg://user?id=441826110\\">АНТОН</a>"}
34	райз(ом)?\\?$	matches	f	{"Слушай если ты еблан который не читал вообще ничего из обсуждений райза, функционала, особенностей и тд - какого хуя вообще ты задаешь этот вопрос из разряда баянов времен неолита?"}
35	(?i)#?скипнул\\s?(весь\\s?)?джерк\\s?на\\s?хуй	matches	f	{"<b>DA TY OHUEL</b>",#читайвесьджерксука}
36	(?i)(([рp][оo][сc][сc]|r[oо]ss) ч[иеe]ни)|^ч[иеe]ни$	matches	f	{хуени}
7	define rust	equals	f	{"<b>Что значит Rust?</b>\\\\n\\\\n- коррозия; ржавчина\\\\n- моральное разложение; коррупция, продажность\\\\n- вредное влияние безделья, бездеятельности (на характер, способности)\\\\n- ухудшаться, портиться, притупляться, вырождаться (от бездействия)\\\\n- притуплять, ослаблять (память, ум)\\\\n- томиться от безделья\\\\n- портить, развращать, разлагать"}
19	(?i)^(borrowck|борроу\\s?чекер|gc|гц)$	matches	t	{"\\"Борроу чекер - это как раз есть то, где человек свободен, потому что он говорит: это нельзя, а все остальное - как хочешь. Что такое гц? Это и есть самая большая несвобода. Я вам могу сказать, что чем больше гц у нас будет, тем менее мы свободны, потому что гц, в отличие от борроу чекера,это когда ты должен действовать, и только таким образом, как написано в Go\\".\\\\n\\\\nРоб Пайк"}
\.


--
-- Data for Name: feeds; Type: TABLE DATA; Schema: public; Owner: ross
--

COPY public.feeds (id, url, kind, timeout, last_update, last_entry) FROM stdin;
3	https://blog.golang.org/feed.atom	atom	3600	2020-03-09 07:04:01.046652	<a href="https://blog.golang.org/a-new-go-api-for-protocol-buffers">A new Go API for Protocol Buffers</a>
1	https://this-week-in-rust.org/rss.xml	rss	3600	2020-03-09 07:04:01.160972	<a href="https://this-week-in-rust.org/blog/2020/03/03/this-week-in-rust-328/">This Week in Rust 328</a>
2	https://blog.rust-lang.org/feed.xml	atom	3600	2020-03-09 07:04:01.25744	<a href="https://blog.rust-lang.org/2020/02/27/Rust-1.41.1.html">Announcing Rust 1.41.1</a>
\.


--
-- Data for Name: greetings; Type: TABLE DATA; Schema: public; Owner: ross
--

COPY public.greetings (id, text) FROM stdin;
1	https://coub.com/view/19ffik
2	Вечер в хату
3	Ас-саля́му ‘але́йкум уа рахматуллах1и уа баракятух1
4	Салам, красава)
5	Слава Україні!
6	გამარჯობა
7	<b>POSHEL NAHUY</b>
8	Ну здравствуй. Присаживайся вон там, возле параши.
9	<b>DOBRO POZHALOVAT NA HUY</b>
10	Гомофобам 👌🎉Вход разрешен!👌🎉
11	А ты что за хуй?
12	<b>Стандартная процедура прописки:</b>\\n\\n1. Кто по жизни, кто по масти?\\n2. Как и откуда о нас узнал(а)?\\n3. На дискуссию или как зритель(ка)?\\n4. Tokio в глаз или в async-std раз?\\n5. Фото своих сисек.
\.


--
-- Data for Name: schedule; Type: TABLE DATA; Schema: public; Owner: ross
--

COPY public.schedule (id, day, "time", messages) FROM stdin;
2	*	20:00:00	{"Победители, как день прошёл, какие горы покорили сегодня?","Ку всем и хорошего вечера, славные войны Гуррен-Данна)","Доброй ночи и прогресса, охуительных идей и просветления."}
3	mon	07:00:00	{"Разъебите эту неделю, Джерковчане","Всем отличной недели, достижения поставленных задач, продуктивной гребли и новых открытий."}
4	thu	12:00:00	{"Как день, пот со лба?"}
5	fri	18:00:00	{"Уважаемые, всем охуенно провести субботу, воскресенье, жизнь. Успехов, роста и производительности."}
6	sun	21:00:00	{"Необщительные вы, ну спать наверное хотите."}
1	*	10:00:00	{"Всем крайне продуктивного дня.","Жерк, доброе утро и всем продуктивной гребли.","Доброе утро, уважаемые разработчики!","Всем продуктивного деградирования и развития, в зависимости от текущих задач))","Уважаемые граждане бандиты, желаю всем сегодня разъебать этот день.","Всем качественного дня/предстоящей ночи)","Здравствуйте дорогое сообщество","Приветствие всем разработчикам на rust)"}
\.


--
-- Data for Name: shippering_phrases; Type: TABLE DATA; Schema: public; Owner: ross
--

COPY public.shippering_phrases (id, template) FROM stdin;
1	Как песок в купальнике, как вино на свадебном платье\\nКак слушать симфонию с выключенным звуком\\nКак гитарист, сломавший руку\\nТакая любовь этих ребят {{first}} + {{last}} = ♥️
2	В горе и в радости\\nВ богатстве и в бедности\\nВ болезни и в здравии\\nКлянутся любить друг друга {{first}} + {{last}} = ♥️
3	Море волнуется раз...\\nМоре волнуется два...\\nМоре волнуется три...\\nИ в любовной позе замирают {{first}} + {{last}} = ♥️
4	Все пройдет и печаль и радость\\nВсе пройдет, так устроен свет\\nВсе пройдет, только верить надо\\nНо между ними любовь не пройдёт {{first}} + {{last}} = ♥️
5	По километрам витой пары\\nЧерез запутанные схемы свичей\\nСквозь блокировки РКН\\nИх связала любовь {{first}} + {{last}} = ♥️
6	Я вас люблю, хоть и бешусь,\\nХоть это труд и стыд напрасный,\\nИ в этой глупости несчастной\\nУ ваших ног я признаюсь!\\n{{first}} + {{last}} = ♥️
7	Желаю новобрачным, чтобы каждый новый день приносил чудесные мгновения радости.\\nНаслаждайтесь жизнью и получайте удовольствие от каждой минуты, проведенной вместе.\\nПусть впечатления от вашей новой совместной жизни будут яркими и красочными.\\nСовершайте вместе маленькие открытия и не уставайте удивлять другу друга.\\nПусть каждая ваша улыбка согревает взаимные чувства\\nПоддерживает огонь желания быть вместе и позволяет сохранить теплоту семейных отношений.\\nЖелаю согласия, любви и счастья вашей семье!\\n{{first}} + {{last}} = ♥️
8	Like Bonny & Clyde..\\nLike Romeo & Julietta..\\nLike Hitler & Eva Braun..\\nThis is also a lovely couple: {{first}} + {{last}} = ♥️
9	Violets are blue,\\nSugar is sweet,\\nand so this lovely couple: {{first}} + {{last}} = ♥️
10	Like Kim Jong Un with nuclear missles..\\nLike Donald Trump & his haircut..\\nLike Snoop Dog & weed..\\nThis is also a lovely couple: {{first}} + {{last}} = ♥️
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: ross
--

COPY public.users (id, first_name, last_name, username) FROM stdin;
448757	nndii	(ya ne Andrey)	nndii
107820725	Kerrigan	\N	sarakerrigan
134759264	Piu	Piu	iampiupiu
26993993	Anton	Rey	Toshiki
115874617	восадули	вогороде	\N
441826110	Антон	\N	insert_reference_here
359010513	весёлый кремовый 🍰 торт	\N	kremovtort
129719794	totallynot@tty	\N	bugworm
289101835	Vladimir	\N	c_vld
360194217	Urry 🌚	Krivorot	jouretz
27122980	Alex	\N	sawaxon
471878788	Tux	\N	tuxubuntu
265303151	Fortunato	\N	fort28
186561677	Alena	Yuryeva	NIMFETRISA
159469089	Nikita	Vilunov	vnikita
187843269	Emmanuel	Goldstein	emmanuelGoldstein
195126013	Panda🤔	\N	PandaThePanda
481434898	Not a Centril	\N	centril
408258968	Hirrolot	\N	hirrolot
108157884	Mike	Lubinets	mersinvald
197333640	Andrew	Demonov	fcoder
3851700	Vlad	0xd728c4a7cd55d8db	razum2um
425276912	Arc<Mutex<S>>	\N	SergeRxx
523792555	cyberbrodyaga 🛰	\N	test3rr
7929120	Cat	\N	kEzViSiOn
138098452	Artjom	\N	thelastwordisrejoice
268486177	p0lunin	\N	p0lunin
7383917	Denis	\N	mexus
380095660	Aikidos	\N	aikidos
597852601	Re-L	\N	re4lmayer
730006604	Григорий	Базукин	\N
292325285	Mikail	Bagishov	MikailBag
101880067	Nikolai	Volkov	HeadcrabInMyRoom
1010004414	Arsenii	Lyashenko	\N
37096931	Evgεny	🤙	fominok
50323043	Seer Iλya	[Vennik E10]	ilyavenner
\.


--
-- Name: autoresponse_phrases_id_seq; Type: SEQUENCE SET; Schema: public; Owner: ross
--

SELECT pg_catalog.setval('public.autoresponse_phrases_id_seq', 36, true);


--
-- Name: feeds_id_seq; Type: SEQUENCE SET; Schema: public; Owner: ross
--

SELECT pg_catalog.setval('public.feeds_id_seq', 3, true);


--
-- Name: greetings_id_seq; Type: SEQUENCE SET; Schema: public; Owner: ross
--

SELECT pg_catalog.setval('public.greetings_id_seq', 12, true);


--
-- Name: schedule_id_seq; Type: SEQUENCE SET; Schema: public; Owner: ross
--

SELECT pg_catalog.setval('public.schedule_id_seq', 6, true);


--
-- Name: shippering_phrases_id_seq; Type: SEQUENCE SET; Schema: public; Owner: ross
--

SELECT pg_catalog.setval('public.shippering_phrases_id_seq', 1, false);


--
-- Name: autoresponse_phrases autoresponse_phrases_pkey; Type: CONSTRAINT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.autoresponse_phrases
    ADD CONSTRAINT autoresponse_phrases_pkey PRIMARY KEY (id);


--
-- Name: feeds feeds_pkey; Type: CONSTRAINT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.feeds
    ADD CONSTRAINT feeds_pkey PRIMARY KEY (id);


--
-- Name: greetings greetings_pkey; Type: CONSTRAINT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.greetings
    ADD CONSTRAINT greetings_pkey PRIMARY KEY (id);


--
-- Name: schedule schedule_pkey; Type: CONSTRAINT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.schedule
    ADD CONSTRAINT schedule_pkey PRIMARY KEY (id);


--
-- Name: shippering_phrases shippering_phrases_pkey; Type: CONSTRAINT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.shippering_phrases
    ADD CONSTRAINT shippering_phrases_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: ross
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

