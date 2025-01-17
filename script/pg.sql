create table public.assets
(
    id             varchar(36) not null
        primary key,
    user_id        varchar(30) not null,
    symbol         varchar(30) not null,
    total_balance  double precision default 0,
    freeze_balance double precision default 0,
    avail_balance  double precision default 0,
    created_at     bigint,
    updated_at     bigint
);

alter table public.assets
    owner to admin;

create unique index user_id_symbol_unique_index
    on public.assets (user_id, symbol);

create unique index userid_symbol
    on public.assets (user_id, symbol);

create table public.assets_logs
(
    id             varchar(36) not null
        primary key,
    user_id        varchar(36) not null,
    symbol         varchar(36) not null,
    before_balance double precision default 0,
    amount         double precision default 0,
    after_balance  double precision default 0,
    trans_id       varchar(36) not null,
    change_type    varchar(36) not null,
    info           varchar(255),
    created_at     bigint,
    updated_at     bigint
);

alter table public.assets_logs
    owner to admin;

create table public.assets_freezes
(
    id            varchar(36)                not null
        primary key,
    user_id       varchar(36)                not null,
    symbol        varchar(36)                not null,
    amount        double precision default 0,
    freeze_amount double precision default 0,
    status        smallint         default 0 not null,
    trans_id      varchar(36)                not null,
    freeze_type   varchar(36)                not null,
    info          varchar(255),
    created_at    bigint,
    updated_at    bigint
);

alter table public.assets_freezes
    owner to admin;

create table public.asset_logs
(
    id             text         not null
        primary key,
    created_at     bigint,
    updated_at     bigint,
    user_id        varchar(30)  not null,
    symbol         varchar(30)  not null,
    before_balance double precision default '0'::numeric,
    amount         double precision default '0'::numeric,
    after_balance  double precision default '0'::numeric,
    trans_id       varchar(100) not null,
    change_type    varchar(15),
    info           varchar(200)
);

alter table public.asset_logs
    owner to admin;

create index idx_asset_logs_user_id
    on public.asset_logs (user_id);

create index idx_trans_id
    on public.asset_logs (trans_id);

create index idx_asset_logs_symbol
    on public.asset_logs (symbol);

create table public.asset_freezes
(
    id            text                                  not null
        primary key,
    created_at    bigint,
    updated_at    bigint,
    user_id       varchar(30)                           not null,
    symbol        varchar(30)                           not null,
    amount        double precision default '0'::numeric not null,
    freeze_amount double precision default '0'::numeric not null,
    status        smallint,
    trans_id      varchar(100)                          not null,
    freeze_type   varchar(15),
    info          varchar(200)
);

alter table public.asset_freezes
    owner to admin;

create unique index trans_id
    on public.asset_freezes (trans_id);

create index idx_asset_freezes_symbol
    on public.asset_freezes (symbol);

create index idx_asset_freezes_user_id
    on public.asset_freezes (user_id);

create table public.varieties
(
    id            serial
        primary key,
    symbol        varchar(100) not null,
    name          varchar(250) not null,
    show_decimals bigint,
    min_decimals  bigint,
    is_base       boolean,
    sort          bigint,
    status        bigint,
    created_at    bigint,
    updated_at    bigint
);

alter table public.varieties
    owner to admin;

create unique index symbol
    on public.varieties (symbol);

create table public.trade_varieties
(
    id               serial
        primary key,
    symbol           varchar(100) not null,
    name             varchar(250) not null,
    target_id        integer default 0,
    base_id          integer default 0,
    price_decimals   bigint  default 2,
    qty_decimals     bigint  default 0,
    allow_min_qty    double precision,
    allow_max_qty    double precision,
    allow_min_amount double precision,
    allow_max_amount double precision,
    fee_rate         double precision,
    status           bigint  default 0,
    sort             bigint  default 0,
    created_at       bigint,
    updated_at       bigint
);

alter table public.trade_varieties
    owner to admin;

create unique index symbol_base_idx
    on public.trade_varieties (target_id, base_id);

create unique index symbol_idx
    on public.trade_varieties (symbol);

create table public.order_auto_no
(
    id           bigserial
        primary key,
    biz_tag      varchar(32) not null
        unique,
    max_id       bigint      not null,
    step         integer     not null,
    description  varchar(255) default NULL::character varying,
    gmt_create   timestamp   not null,
    gmt_modified timestamp   not null
);

comment on table public.order_auto_no is '订单编号表';

comment on column public.order_auto_no.id is '主键';

comment on column public.order_auto_no.biz_tag is '业务标识';

comment on column public.order_auto_no.max_id is '最大ID';

comment on column public.order_auto_no.step is '步长';

comment on column public.order_auto_no.description is '备注';

comment on column public.order_auto_no.gmt_create is '创建时间';

comment on column public.order_auto_no.gmt_modified is '更新时间';

alter table public.order_auto_no
    owner to admin;

create table public."order"
(
    uuid            varchar(255)               not null
        primary key,
    base            varchar                    not null,
    symbol          varchar(30)                not null,
    order_id        varchar(30)                not null
        unique,
    order_side      varchar(10)                not null,
    order_type      varchar(10)                not null,
    user_id         varchar(64)                not null,
    price           double precision default 0 not null,
    quantity        double precision default 0 not null,
    fee_rate        double precision default 0 not null,
    amount          double precision default 0 not null,
    freeze_qty      double precision default 0 not null,
    freeze_amount   double precision default 0 not null,
    avg_price       double precision default 0 not null,
    finished_qty    double precision default 0 not null,
    finished_amount double precision default 0 not null,
    fee             double precision default 0 not null,
    status          smallint         default 0 not null,
    nano_time       bigint           default 0 not null
);

alter table public."order"
    owner to admin;

create table public.trade_log
(
    uuid         varchar          not null
        primary key,
    base         varchar          not null,
    symbol       varchar          not null,
    trade_id     varchar          not null,
    ask          varchar          not null,
    bid          varchar          not null,
    trade_by     smallint         not null,
    ask_uid      varchar          not null,
    bid_uid      varchar          not null,
    price        double precision not null,
    quantity     double precision not null,
    amount       double precision not null,
    ask_fee_rate double precision not null,
    ask_fee      double precision not null,
    bid_fee_rate double precision not null,
    bid_fee      double precision not null
);

alter table public.trade_log
    owner to admin;

create table public.kline
(
    uuid     varchar(255)               not null
        primary key,
    base     varchar                    not null,
    symbol   varchar                    not null,
    period   varchar                    not null,
    open_at  bigint,
    close_at bigint,
    open     double precision default 0 not null,
    high     double precision default 0 not null,
    low      double precision default 0 not null,
    close    double precision default 0 not null,
    volume   double precision default 0 not null,
    amount   double precision default 0 not null,
    unique (open_at, close_at)
);

alter table public.kline
    owner to admin;

