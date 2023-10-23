package com.micronaut.model;

import io.micronaut.data.annotation.GeneratedValue;
import io.micronaut.data.annotation.Id;
import io.micronaut.data.annotation.MappedEntity;
import io.micronaut.serde.annotation.Serdeable;
import lombok.Getter;
import lombok.NonNull;
import lombok.RequiredArgsConstructor;
import lombok.Setter;

import java.io.Serializable;
import java.math.BigDecimal;

@Setter
@Getter
@RequiredArgsConstructor
@MappedEntity
@Serdeable
public class Client implements Serializable {

    @GeneratedValue
    @Id
    private Long id;
    private @NonNull String name;
    private @NonNull int age;
    private @NonNull String address;
    private @NonNull double salary;
}
