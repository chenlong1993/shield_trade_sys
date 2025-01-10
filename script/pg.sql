create table public.assets
(
    id             varchar(36) not null
        primary key,
    user_id        varchar(30) not null,
    symbol         varchar(30) not null,
    total_balance  numeric(40, 20) default 0,
    freeze_balance numeric(40, 20) default 0,
    avail_balance  numeric(40, 20) default 0,
    created_at     timestamp with time zone,
    updated_at     timestamp with time zone
);

alter table public.assets
    owner to admin;

create unique index user_id_symbol_unique_index
    on public.assets (user_id, symbol);

create unique index userid_symbol
    on public.assets (user_id, symbol);

create table public.assets_logs
(
    id             varchar(36)                            not null
        primary key,
    user_id        varchar(36)                            not null,
    symbol         varchar(36)                            not null,
    before_balance numeric(40, 20)          default 0,
    amount         numeric(40, 20)          default 0,
    after_balance  numeric(40, 20)          default 0,
    trans_id       varchar(36)                            not null,
    change_type    varchar(36)                            not null,
    info           varchar(255),
    created_at     timestamp with time zone default now() not null,
    updated_at     timestamp with time zone default now() not null
);

alter table public.assets_logs
    owner to admin;

create table public.assets_freezes
(
    id            varchar(36)                            not null
        primary key,
    user_id       varchar(36)                            not null,
    symbol        varchar(36)                            not null,
    amount        numeric(40, 20)          default 0,
    freeze_amount numeric(40, 20)          default 0,
    status        smallint                 default 0     not null,
    trans_id      varchar(36)                            not null,
    freeze_type   varchar(36)                            not null,
    info          varchar(255),
    created_at    timestamp with time zone default now() not null,
    updated_at    timestamp with time zone default now() not null
);

alter table public.assets_freezes
    owner to admin;

create table public.asset_logs
(
    id             text         not null
        primary key,
    created_at     timestamp with time zone,
    updated_at     timestamp with time zone,
    user_id        varchar(30)  not null,
    symbol         varchar(30)  not null,
    before_balance numeric(40, 20) default '0'::numeric,
    amount         numeric(40, 20) default '0'::numeric,
    after_balance  numeric(40, 20) default '0'::numeric,
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
    id            text                                 not null
        primary key,
    created_at    timestamp with time zone,
    updated_at    timestamp with time zone,
    user_id       varchar(30)                          not null,
    symbol        varchar(30)                          not null,
    amount        numeric(40, 20) default '0'::numeric not null,
    freeze_amount numeric(40, 20) default '0'::numeric not null,
    status        smallint,
    trans_id      varchar(100)                         not null,
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
    created_at    timestamp with time zone,
    updated_at    timestamp with time zone
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
    target_id        integer         default 0,
    base_id          integer         default 0,
    price_decimals   bigint          default 2,
    qty_decimals     bigint          default 0,
    allow_min_qty    numeric(40, 20) default 0.01,
    allow_max_qty    numeric(40, 20) default '999999'::numeric,
    allow_min_amount numeric(40, 20) default 0.01,
    allow_max_amount numeric(40, 20) default '999999'::numeric,
    fee_rate         numeric(40, 20) default '0'::numeric,
    status           bigint          default 0,
    sort             bigint          default 0,
    created_at       timestamp with time zone,
    updated_at       timestamp with time zone
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

